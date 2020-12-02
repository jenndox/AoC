
Place Ruby scripts here.

1) Add a shebang line at the top of your script:

```bash
#!/apollo/sbin/envroot $ENVROOT/bin/ruby
```

2) Build your package:

```bash
brazil-build release
```

3) Run your scripts

.. using build artifacts like this:

```bash
brazil-runtime-exec my_script.rb
```

.. or using brazil-bootstrap like this:

```bash
$(brazil-bootstrap)/bin/my_script.rb
```
