#!/bin/bash

chonkify_commit_push() {
  msg="$@"

  git add .; \
  git commit -m "$msg" && \
  git push -u all master
}

chonkify_commit_push "$@"
