# boringfetch

Node.js bindings to [boringhyper](https://github.com/makindotcc/boringhyper) allowing to request https server with
Chrome tls fingerprint ja3.

## Installing boringfetch

Installing boringfetch requires a [supported version of Node and Rust](https://github.com/neon-bindings/neon#platform-support).

You can install the project with npm. In the project directory, run:

```sh
$ npm install
```

This fully installs the project, including installing any dependencies and running the build.

## Building boringfetch

If you have already installed the project and only want to run the build, run:

```sh
$ npm run build
```

This command uses the [cargo-cp-artifact](https://github.com/neon-bindings/cargo-cp-artifact) utility to run the Rust build and copy the built library into `./index.node`.

## Exploring boringfetch

After building boringfetch, you can explore its exports at the Node REPL:

```sh
$ npm install
$ node
> require('.').boringfetch("https://www.canva.com/pl_pl/").then(console.log)
Promise {
  <pending>,
  [Symbol(async_id_symbol)]: 63,
  [Symbol(trigger_async_id_symbol)]: 62
}
> {
  status: 200,
  body: '<!DOCTYPE html><html dir="ltr" lang="pl-PL">[...]',
}
```
