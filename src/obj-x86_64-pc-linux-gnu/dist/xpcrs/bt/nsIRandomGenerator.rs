//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIRandomGenerator.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIRandomGenerator",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void generateRandomBytes (in unsigned long aLength, [array, size_is (aLength), retval] out octet aBuffer); */
                    Method {
                        name: "GenerateRandomBytes",
                        params: &[Param { name: "aLength", ty: "u32" }, Param { name: "aBuffer", ty: "*mut *mut u8" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

