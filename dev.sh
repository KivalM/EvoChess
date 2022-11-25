export PATH="$HOME/.cargo/bin:$PATH"

# cargo watch -w server -w src -x "run"
# cd app
# trunk watch

cargo watch -w server -w src -x "run" & cd app;trunk watch && fg