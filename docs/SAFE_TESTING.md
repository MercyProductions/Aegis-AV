# Safe Testing

Use these test categories only:

- EICAR antivirus test string
- fake hash signatures
- dummy executable-like files
- harmless scripts
- empty files
- large generated files
- folders with expected permission errors

The default signature file contains the SHA256 hash of the canonical EICAR test string:

```txt
275a021bbfb6489e54d471899f7db9d1663fc695ec2fe2a2c4538aabf651fd0f
```

Some host antivirus products immediately quarantine EICAR files. If you intentionally create one, do it in a controlled test folder and expect your existing antivirus to alert.
