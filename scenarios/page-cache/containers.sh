#!/bin/bash

set -o errexit

readonly GROUP_NAME="testcg"

main () {
	case $1 in
		create)
			create
			;;

		delete)
			delete
			;;

		*)
			echo "usage: (create|delete)"
			exit 1
			;;
	esac;
}

delete () {
	for i in $(seq 1 10); do
		delete_cgroup $i
	done

	rmdir /sys/fs/cgroup/memory/$GROUP_NAME
}

create () {
	mkdir -p /sys/fs/cgroup/memory/$GROUP_NAME

	for i in $(seq 1 10); do
		create_cgroup $i
	done
}

delete_cgroup () {
	local index=$1

	rmdir /sys/fs/cgroup/memory/$GROUP_NAME/$index
}

create_cgroup () {
	local index=$1
	local dest_dir=/tmp/testcg/$index

	mkdir /sys/fs/cgroup/memory/$GROUP_NAME/$index
	echo "$$" > /sys/fs/cgroup/memory/$GROUP_NAME/$index/cgroup.procs

	mkdir -p $dest_dir
	mount -t tmpfs $index $dest_dir
	touch $dest_dir/file
}

main "$@"
