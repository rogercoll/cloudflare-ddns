#!/bin/bash

if [[ -z "${TOKEN}" && -z "${RECORD_ID}" && -z "${ZONE_ID}" ]]; then
        echo "TOKEN, RECORD_ID and ZONE_ID environment variables must be defined"
        exit 1
fi

if [[ -z "${LONG_RUNNING}" ]]; then
        exec /target/debug/cloudflare-ddns --token ${TOKEN} --record-name ${RECORD_NAME} --zone-id ${ZONE_ID}
  else
        exec /target/debug/cloudflare-ddns --token ${TOKEN} --record-name ${RECORD_NAME} --zone-id ${ZONE_ID} --long-running ${LONG_RUNNING}
fi
