cargo new --bin server --vcs none \
    && cargo new --bin frontend --vcs none \
    && cargo new --lib shared --vcs none \
    && echo "[workspace]\nmembers = [\"server\", \"frontend\", \"shared\"]" \
    | cat > Cargo.toml \
    && echo "/target\n/dist" | cat > .gitignore