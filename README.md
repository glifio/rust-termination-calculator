rust-termination-calculator
=========================

Usage:

```
cargo run -- \
  --epoch 3559748 \
  --sector-size 32 \
  --qap-position=9759082362841844682881538327065773703263060121749055791461 \
  --qap-velocity=-7580969881544121507823389406846038852149922941494925 \
  --reward-position=16782941870422397609460720690127419622109456322625328327505 \
  --reward-velocity=-26722374235001584454611811757655045006281911162321526 \
  --activation 3395382 \
  --expiration 4944643 \
  --deal-weight 0 \
  --verified-deal-weight 0 \
  --expected-day-reward 188054129953956 \
  --expected-storage-pledge 3707397053860264 \
  --power-base-epoch 0 \
  --replaced-day-reward 0
    Finished dev [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/rust-termination-calculator --epoch 3559748 --sector-size 32 --qap-position 9759082362841844682881538327065773703263060121749055791461 --qap-velocity=-7580969881544121507823389406846038852149922941494925 --reward-position 16782941870422397609460720690127419622109456322625328327505 --reward-velocity=-26722374235001584454611811757655045006281911162321526 --activation 3395382 --expiration 4944643 --deal-weight 0 --verified-deal-weight 0 --expected-day-reward 188054129953956 --expected-storage-pledge 3707397053860264 --power-base-epoch 0 --replaced-day-reward 0`
16871186150637184
```

# See also

* https://github.com/glifio/go-termination-calculator

# License

Apache 2
