# Swarmcret

_A Docker Swarm secret adapter_

[![Build](https://github.com/xstefanox/swarmcret/actions/workflows/build.yml/badge.svg)](https://github.com/xstefanox/swarmcret/actions/workflows/build.yml)
![Docker Image Version (latest semver)](https://img.shields.io/docker/v/xstefanox/swarmcret?label=Docker%20Hub&sort=semver)

## Description

The best practice to pass configuration to an application is using environment variables, as
described in [12 Factor App](https://12factor.net/config): this ensures strict separation of
configuration from code and allows easy configuration changes between deployments.

This is possible with Kubernetes by
[mounting secrets as environment variables](https://kubernetes.io/docs/concepts/configuration/secret/#using-secrets-as-environment-variables).
Docker Swarm is currently lacking this feature, because it only allows to mount
[secrets](https://docs.docker.com/engine/swarm/secrets/) and
[configs](https://docs.docker.com/engine/swarm/configs/) in the container filesystem.

Swarmcret is a solution that can be used to adapt configuration mounted in the filesystem into a set
of environment variables.

## Origin of the name

"Swarmcret" is the portmanteau of "swarm" and "secret" and it is also the Frison translation for "swarm".

## How to use

1. Mount your secrets in `/var/run/secrets` path
2. Mount your configs in `/var/run/configs` path
3. Load the Swarmcret Docker image into a multi-stage Dockerfile:

   ```dockerfile
   FROM xstefanox/swarmcret:1.0 as swarmcret

   # then in the production stage...
   COPY --from=swarmcret /swarmcret /usr/local/bin/swarmcret
   ```
4. Put Swarmcret in the image entrypoint

   Standalone example:
   ```dockerfile
   ENTRYPOINT [ "swarmcret"]
   ```

   Example with [Tini](https://github.com/krallin/tini):
   ```dockerfile
   ENTRYPOINT [ "tini", "--", "swarmcret"]
   ```

## How it works

Swarmcret scans the secrets and configs directories and convert each file into an environment
variable having the same name of the file and the file content as value.

### Example

Given the following secret mounted in the filesystem
```
/var/run/secrets/MY_SECRET
```

whose value is `the_secret_value`, Swarmcret will convert it into the following environment variable

```shell
MY_SECRET=the_secret_value
```

It then starts the command defined in the `CMD` statement of the Dockerfile.
Since this process is created with execve syscall, it will inherit the signal handlers of Swarmcret
itself: this ensures the compatibility with Tini (or other container init processes).
