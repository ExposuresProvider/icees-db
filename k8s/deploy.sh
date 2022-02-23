#!/usr/bin/env bash

kubectl --namespace $1 apply -f iceesdb-deployment.yaml
