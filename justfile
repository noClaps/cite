build:
    @monopack src/index.html

serve:
    @watchexec --clear --restart --ignore dist 'just build && serve dist'
