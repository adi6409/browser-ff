//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/ds/nsISerializable.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISerializable",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [must_use] void read (in nsIObjectInputStream aInputStream); */
                    Method {
                        name: "Read",
                        params: &[Param { name: "aInputStream", ty: "*const nsIObjectInputStream" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void write (in nsIObjectOutputStream aOutputStream); */
                    Method {
                        name: "Write",
                        params: &[Param { name: "aOutputStream", ty: "*const nsIObjectOutputStream" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

