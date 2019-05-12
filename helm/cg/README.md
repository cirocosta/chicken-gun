# cg

Wraps `chicken-gun` as a Helm chart.

## Examples

### Raw network throughput

Consider the case in which testing the network throughput between a receiver and a transmitter is desired.

That can be accomplished by creating two releases of `cg`, one for the receiver, another for the transmitter:

- **receiver**

```yaml
nodeSelector:
  kubernetes.io/hostname: gke-hush-house-workers-2-11ce616a-719v
args:
  - tcp-receiver
  - --address=0.0.0.0:1337
```

- **transmitter**

```yaml
nodeSelector:
  kubernetes.io/hostname: gke-hush-house-workers-1-332140f7-6bzn

args:
  - tcp-transmitter
  - --address=receiver-cg:1337
```


### Network to disk throughput

Now, consider that we want to check what's the throughput acheivable when we take bits from the network and write those down to a file on disk.

This would require us changing the receiver side of the ["Raw Network Throughput"](#raw-network-throughput) example to write to a particular location on disk.

Assuming that we want to write to a disk that gets mounted onto the container using a Persistent Volume Claim, we can do so like the following:


```yaml
nodeSelector:
  kubernetes.io/hostname: gke-hush-house-workers-2-11ce616a-719v

persistence:
  enabled: true
  size: 1000Gi
  storageClassName: ssd

args:
  - tcp-receiver
  - --address=0.0.0.0:1337
  - --destination=/mnt/pd/file.txt
```

However, if we wanted to test the throughput of writing to a particular mount on the filesystem, then we'd switch back to not requiring persistence (as that translates to using PVCs), and, instead, specifying volumes and volume mounts:


``` yaml
nodeSelector:
  kubernetes.io/hostname: gke-hush-house-workers-2-11ce616a-719v

additionalVolumes:
  - name: local-mount
    hostPath:
      path: /mnt/disks/array
      type: Directory

additionalVolumeMounts:
  - name: local-mount
    mountPath: /mnt/pd

args:
  - tcp-receiver
  - --address=0.0.0.0:1337
  - --destination=/mnt/pd/file.txt
```

