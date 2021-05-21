//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsISiteSecurityService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISiteSecurityState",
            base: Some("nsISupports"),
            methods: Err("specialtype jsval unsupported"),
        },

        Interface {
            name: "nsISiteHSTSState",
            base: Some("nsISiteSecurityState"),
            methods: Ok(&[
                    ]),
        },

        Interface {
            name: "nsISiteSecurityService",
            base: Some("nsISupports"),
            methods: Err("native type const mozilla::OriginAttributes unsupported"),
        },

        ]; D}

