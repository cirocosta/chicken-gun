#!/bin/bash

set -o errexit
set -o xtrace

main () {
	create_files
}


create_files () {
	mkdir -p /tmp/files
	for i in $(seq 1 5); do
		dd bs="$((1024 * 1024))" count=1024 if=/dev/zero of=/tmp/files/file-$i
	done
	sync
}

main "$@"
