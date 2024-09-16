# Salve Mundi website redesign met leptos en tailwind

Voor media moet ik een website of app (her)uitvinden, en de site van de studivereniging leek me een
leuk doelwit. Ik heb met een aantal mensen gesproken, ik heb een survey weggestuurd en ben met die
input aan de slag gegaan.

## Lokaal bouwen

1. Installeer rustup:
    [rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

2. Installeer rust nightly met de wasm32-unknown-unknown target:

    ```sh
    rustup install nightly
    rustup target add wasm32-unknown-unknown
    ```

3. Installeer trunk:

    ```sh
    cargo install trunk
    ```

Met deze tools kun je nu `trunk serve` in de root directory runnen. Voor meer informatie over trunk
bestaat [trunkrs.dev](https://trunkrs.dev/).
