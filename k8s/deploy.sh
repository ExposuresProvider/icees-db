#!/usr/bin/env bash

# run it by passing in namespace as an argument
# e.g., ./deploy.sh icees-dev
kubectl --namespace $1 apply -f iceesdb-pod.yaml
