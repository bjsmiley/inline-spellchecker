# Inline Spell Checker
A small tool I made instead of sleeping. Motivation: to quickly auto-correct words I know I'm about to butcher instead of going to google to correct it.

## Example
```shell
> git commit -m "fixed json $(sp desearialization)"
[master 4c7b743] fixed json deserialization
 1 file changed, 2 insertions(+), 3 deletions(-)
>
```

## Implementation
I used the windows api, because why not. ~~Can~~ Should I even extend this to [macos](https://github.com/ryanmcgrath/cacao) next?