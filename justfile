@list-objects:
    find .git/objects -type f

@view-objects:
    git cat-file --batch-check --batch-all-objects
