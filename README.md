# UnityAssetBundleOverview

A Rust tool to deserialize AssetBundle file of Unity.

It is degraded copy of Mikunyan for my learning Rust lang.

About Mikunyan, see > http://www.rubydoc.info/gems/mikunyan/

Mikunyan is a great ruby library to deserialize AssetBundle file of Unity.

# How To Use

```
$ uabe --src /path/to/foo.unity3d --dst /path/to/your.json
$ cat /path/to/your.json
{
  "signiture": "UnityFS",
  "file_version": 6,
  "lower_player_version": "5.x.x",
  "upper_player_version": "2018.x.xfx",
  ...
```

