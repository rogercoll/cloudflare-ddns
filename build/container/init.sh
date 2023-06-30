#!/bin/bash

if [[ -z "${TOKEN}" && -z "${RECORD_ID}" && -z "${ZONE_ID}" ]]; then
        echo "TOKEN, RECORD_ID and ZONE_ID environment variables must be defined"
        exit 1
fi

GLOBAL_ARGS=""

if [[ "${LONG_RUNNING}" ]]; then
    GLOBAL_ARGS="$GLOBAL_ARGS --long-running $LONG_RUNNING"
fi

if [[ "${IP_CHECKER}" ]]; then
    GLOBAL_ARGS="$GLOBAL_ARGS --ip_checker $IP_CHECKER"
fi

exec /target/debug/cloudflare-ddns --token ${TOKEN} --record-name ${RECORD_NAME} --zone-id ${ZONE_ID} ${GLOBAL_ARGS}
