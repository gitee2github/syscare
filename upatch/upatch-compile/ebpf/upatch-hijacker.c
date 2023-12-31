// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
/* Copyright (c) 2023 Longjun Luo */

#include <stdio.h>
#include <unistd.h>
#include <signal.h>
#include <string.h>
#include <errno.h>
#include <fcntl.h>
#include <limits.h>
#include <getopt.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <sys/socket.h>
#include <sys/un.h>

#include <sys/resource.h>
#include <bpf/libbpf.h>

#include "upatch-hijacker.skel.h"
#include "upatch-entry.h"
#include "upatch-socket.h"

static char path_buff[PATH_MAX];
static volatile sig_atomic_t stop;
static int upatch_socket = -1;
static struct upatch_hijacker_bpf *skel = NULL;

static void sig_int(int signo)
{
	stop = 1;
	if (upatch_socket != -1)
		close(upatch_socket);
	upatch_socket = -1;
}

static void entry_des_create(struct upatch_entry_des *entry_des, unsigned long entry_ino,
	const char *jumper_name, bool if_hijacker)
{
	entry_des->ref = 1;
	entry_des->if_hijacker = if_hijacker;
	entry_des->self_ino = entry_ino;
	strncpy((char *)&entry_des->jumper_path, jumper_name,
		sizeof(entry_des->jumper_path) - 1);
}

static inline int entry_lookup_by_name(const char *entry_name,
	struct upatch_entry *entry, struct upatch_entry_des *entry_des)
{
	const struct bpf_map *map = skel->maps.hijacker_entries;
	memset(entry, 0, sizeof(*entry));
	strncpy(entry->name, entry_name, sizeof(entry->name) - 1);
	return bpf_map__lookup_elem(map, entry, sizeof(*entry),
		entry_des, sizeof(*entry_des), 0);
}

static int entry_put(const char *entry_name)
{
	int ret;
	unsigned int entry_len;
	struct upatch_entry entry;
	struct upatch_entry_des entry_des;
	const struct bpf_map *map = skel->maps.hijacker_entries;

	entry_len = strlen(entry_name);
	if (entry_len + 1 > UPATCH_ENTRY_MAX_LEN)
		return -EINVAL;

	ret = entry_lookup_by_name(entry_name, &entry, &entry_des);
	if (ret)
		return ret;
	
	if (entry_des.ref == 0)
		return -EPERM;

	/* ATTENTION: since we have no lock between kernel and user, no delete */
	entry_des.ref --;
	return bpf_map__update_elem(map, &entry, sizeof(entry),
			&entry_des, sizeof(entry_des), 0);
}

static int entry_get(unsigned long entry_ino, const char *entry_name,
	const char *jumper_name, bool if_hijacker)
{
	int ret;
	unsigned int entry_len, jumper_len;
	struct upatch_entry entry;
	struct upatch_entry_des entry_des;
	const struct bpf_map *map = skel->maps.hijacker_entries;

	entry_len = strlen(entry_name);
	jumper_len = strlen(jumper_name);

	if (entry_len + 1 > UPATCH_ENTRY_MAX_LEN || jumper_len + 1 > UPATCH_ENTRY_MAX_LEN)
		return -EINVAL;

	ret = entry_lookup_by_name(entry_name, &entry, &entry_des);
	if (ret == 0)
		entry_des.ref ++;
	else if (ret == -ENOENT)
		entry_des_create(&entry_des, entry_ino, jumper_name, if_hijacker);
	else
		return ret;

	if (strcmp((char *)&entry_des.jumper_path, jumper_name) != 0
        || entry_des.if_hijacker != if_hijacker)
		return -EPERM;

	return bpf_map__update_elem(map, &entry, sizeof(entry),
		&entry_des, sizeof(entry_des), 0);
}

static int register_entry(unsigned long prey_ino, const char *prey_name,
	unsigned long hijacker_ino, const char *hijacker_name)
{
	int ret;

	/* check soft link first */
	ret = readlink(hijacker_name, (char *)&path_buff, PATH_MAX);
	if (ret == -1)
		return -errno;
	path_buff[ret] = '\0';

	ret = entry_get(prey_ino, prey_name, hijacker_name, 0);
	if (ret)
		goto out;

	ret = entry_get(hijacker_ino, (char *)&path_buff, prey_name, 1);
	if (ret)
		goto out_clean;

	goto out;
out_clean:
	entry_put(prey_name);
out:
	return ret;
}

static inline bool check_if_hijacker(const char *path, unsigned long hijacker_ino)
{
    struct stat path_lstat, path_stat;

    /* check if it is a link to the hijacker */
    if (lstat(path, &path_lstat) == -1 || stat(path, &path_stat) == -1)
        return false;

    if (S_ISLNK(path_lstat.st_mode) && path_stat.st_ino == hijacker_ino)
        return true;

    return false;
}

static inline bool find_hijacker_range(char start, char end, char *path,
	const char *hijacker_path, unsigned long hijacker_ino)
{
	for (path[1] = start; path[1] != end + 1; path[1] ++) {
		/* if file exists, check if it is the hijacker. */
		if (access(path, F_OK) == 0 && check_if_hijacker(path, hijacker_ino))
			return true;
		else if (symlink(hijacker_path, path) == 0)
			return true;
	}
	return false;
}

static int establish_hijacker_link(char *buff, int buff_len,
	const char *hijacker_path, unsigned long hijacker_ino)
{
	if (buff_len < 3)
		return -EINVAL;

	if (find_hijacker_range('A', 'Z', buff, hijacker_path, hijacker_ino))
		return 0;

	if (find_hijacker_range('a', 'z', buff, hijacker_path, hijacker_ino))
		return 0;

	return -EMLINK;
}

int upatch_register_entry(unsigned long prey_ino, const char *prey_name,
	unsigned long hijacker_ino, const char *hijacker_name)
{
	int ret;
	char path[] = "/1";

	ret = establish_hijacker_link((char *)&path, sizeof(path),
		hijacker_name, hijacker_ino);
	if (ret)
		return ret;

	printf("find link path %s for %s \n", path, hijacker_name);
	ret = register_entry(prey_ino, prey_name, hijacker_ino, (char *)&path);
	if (ret)
		return ret;

	skel->bss->hijacker_total_ref ++;
	return 0;
}

int upatch_unregister_entry(const char *prey_name, const char *hijacker_name)
{
	if (skel->bss->hijacker_total_ref == 0)
		return -EPERM;

	entry_put(prey_name);
	entry_put(hijacker_name);
	skel->bss->hijacker_total_ref --;
	return 0;
}

static int libbpf_print_fn(enum libbpf_print_level level, const char *format, va_list args)
{
	return vfprintf(stderr, format, args);
}

static int socket_init()
{
	int ret;
	struct sockaddr_un addr;
	upatch_socket = socket(AF_UNIX, SOCK_STREAM, 0);
	if (upatch_socket == -1)
		return -errno;

	memset(&addr, 0, sizeof(struct sockaddr_un));
	addr.sun_family = AF_UNIX;
	strncpy(addr.sun_path, UPATCH_SOCKET_PATH, sizeof(addr.sun_path) - 1);

	ret = bind(upatch_socket, (const struct sockaddr *) &addr,
		sizeof(struct sockaddr_un));
	if (ret == -1)
		return -errno;

	ret = listen(upatch_socket, UPATCH_MAX_HIJACK_ENTRY);
	if (ret == -1)
		return -errno;
	return 0;
}

static void handle_socket(int data_socket)
{
	int ret, execute_res;
	struct upatch_socket_msg buff;
	if (data_socket == -1)
		goto out;

	ret = read(data_socket, &buff, sizeof(buff));
	if (ret != sizeof(buff) || buff.magic != UPATCH_SOCKET_MAGIC) {
		fprintf(stderr, "wrong size or vertify magic failed \n");
		goto out;
	}

	if (buff.hijacker_ino == 0 && buff.prey_ino == 0)
		execute_res = upatch_unregister_entry((char *)&buff.prey_name,
			(char *)&buff.hijacker_name);
	else
		execute_res = upatch_register_entry(buff.prey_ino, (char *)&buff.prey_name,
			buff.hijacker_ino, (char *)&buff.hijacker_name);

	ret = write(data_socket, &execute_res, sizeof(execute_res));
	if (ret != sizeof(execute_res)) {
		fprintf(stderr, "write socket failed - %d \n", ret);
		goto out;
	}
out:
	if (data_socket != -1)
		close(data_socket);
	return;
}

static void clean_socket()
{
	if (upatch_socket != -1)
		close(upatch_socket);
	unlink(UPATCH_SOCKET_PATH);
}

int main(int argc, char **argv)
{
	int err;

	/* Set up libbpf errors and debug info callback */
	libbpf_set_print(libbpf_print_fn);

	/* Open load and verify BPF application */
	skel = upatch_hijacker_bpf__open();
	if (!skel) {
		fprintf(stderr, "Failed to open BPF skeleton\n");
		return 1;
	}

	err = upatch_hijacker_bpf__load(skel);
	if (err) {
		fprintf(stderr, "Failed to load and verify BPF skeleton\n");
		goto cleanup;
	}

	err = socket_init();
	if (err) {
		fprintf(stderr, "Init error failed - %d \n", err);
		goto cleanup;
	}

	/* Attach tracepoint handler */
	err = upatch_hijacker_bpf__attach(skel);
	if (err) {
		fprintf(stderr, "Failed to attach BPF skeleton\n");
		goto cleanup;
	}

	if (signal(SIGINT, sig_int) == SIG_ERR) {
		fprintf(stderr, "can't set signal handler: %s\n", strerror(errno));
		goto cleanup;
	}

	while (!stop) {
		handle_socket(accept(upatch_socket, NULL, NULL));
	}

	printf("hijack end, clean the resource \n");
	clean_socket();
	upatch_hijacker_bpf__detach(skel);
cleanup:
	upatch_hijacker_bpf__destroy(skel);
	return -err;
}

