use hex_impl::hex;

#[test]
fn from_hex() {
    assert_eq!(
        hex!(#f79489),
        [
            0xf7 as f32 / 255.0,
            0x94 as f32 / 255.0,
            0x89 as f32 / 255.0,
            1.0
        ]
    );

    const C: [f32; 4] = hex!("#7ec8e390");
    assert_eq!(
        C,
        [
            0x7e as f32 / 255.0,
            0xc8 as f32 / 255.0,
            0xe3 as f32 / 255.0,
            0x90 as f32 / 255.0
        ]
    );

    static E: [f32; 4] = hex!(fD49a0);
    assert_eq!(
        E,
        [
            0xfd as f32 / 255.0,
            0x49 as f32 / 255.0,
            0xa0 as f32 / 255.0,
            1.0
        ]
    );

    assert_eq!(
        hex![
            fD49a0,
            "#7ec8e390",
            #f79489,
        ],
        [
            [
                0xfd as f32 / 255.0,
                0x49 as f32 / 255.0,
                0xa0 as f32 / 255.0,
                1.0
            ],
            [
                0x7e as f32 / 255.0,
                0xc8 as f32 / 255.0,
                0xe3 as f32 / 255.0,
                0x90 as f32 / 255.0
            ],
            [
                0xf7 as f32 / 255.0,
                0x94 as f32 / 255.0,
                0x89 as f32 / 255.0,
                1.0
            ]
        ]
    )
}

#[test]
fn from_hex_bad_format() {
    assert_eq!(
        hex!("f7948 9"),
        [
            0xf7 as f32 / 255.0,
            0x94 as f32 / 255.0,
            0x89 as f32 / 255.0,
            1.0
        ]
    );

    assert_eq!(
        hex!("7e c8e390"),
        [
            0x7e as f32 / 255.0,
            0xc8 as f32 / 255.0,
            0xe3 as f32 / 255.0,
            0x90 as f32 / 255.0
        ]
    );

    assert_eq!(
        hex!(7 e c8 e3  " 9 "0),
        [
            0x7e as f32 / 255.0,
            0xc8 as f32 / 255.0,
            0xe3 as f32 / 255.0,
            0x90 as f32 / 255.0
        ]
    );

    assert_eq!(
        hex!(fd4 9a0),
        [
            0xfd as f32 / 255.0,
            0x49 as f32 / 255.0,
            0xa0 as f32 / 255.0,
            1.0
        ]
    );

    assert_eq!(
        hex![
            "f7948 9",
            "7e c8e390",
            7 e c8 e3  " 9 "0,
            fd4 9a0
        ],
        [
            [
                0xf7 as f32 / 255.0,
                0x94 as f32 / 255.0,
                0x89 as f32 / 255.0,
                1.0
            ],
            [
                0x7e as f32 / 255.0,
                0xc8 as f32 / 255.0,
                0xe3 as f32 / 255.0,
                0x90 as f32 / 255.0
            ],
            [
                0x7e as f32 / 255.0,
                0xc8 as f32 / 255.0,
                0xe3 as f32 / 255.0,
                0x90 as f32 / 255.0
            ],
            [
                0xfd as f32 / 255.0,
                0x49 as f32 / 255.0,
                0xa0 as f32 / 255.0,
                1.0
            ]
        ]
    )
}
