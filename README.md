# mm4cc-web

My website (hosted on https://maddiem4.cc/).

How to set up (assumuming you've cloned the repo and have a CF tunnel token):

```bash
# Set up the pages symlink to your Obsidian vault (location varies per machine):
ln -s ~/vault pages

# Configure the tunnel
make dev.env
vim dev.env # Fix tunnel token with real value

# Setup done! To run in the foreground:
make up

# To run in the background:
make upd
```
