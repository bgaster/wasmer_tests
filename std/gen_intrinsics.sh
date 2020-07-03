bindgen src/intrinsics.h \
    --output src/intrinsics.rs \
    --use-core \
     --raw-line '#![allow(bad_style, dead_code)]'