#!/bin/bash

gcloud run deploy --project chonkify \
    --image gcr.io/chonkify/chonkify:latest \
    --platform managed \
    --region us-central1 \
    --max-instances 1 \
    --memory 256Mi \
    --allow-unauthenticated \
    chonkify
