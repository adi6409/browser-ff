//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/sandbox/common/mozISandboxSettings.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "mozISandboxSettings",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute long effectiveContentSandboxLevel; */
                    Method {
                        name: "GetEffectiveContentSandboxLevel",
                        params: &[Param { name: "aEffectiveContentSandboxLevel", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

