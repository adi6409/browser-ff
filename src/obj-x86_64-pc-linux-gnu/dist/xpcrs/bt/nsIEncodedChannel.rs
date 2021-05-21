//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIEncodedChannel.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIEncodedChannel",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute nsIUTF8StringEnumerator contentEncodings; */
                    Method {
                        name: "GetContentEncodings",
                        params: &[Param { name: "aContentEncodings", ty: "*mut*const nsIUTF8StringEnumerator" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean applyConversion; */
                    Method {
                        name: "GetApplyConversion",
                        params: &[Param { name: "aApplyConversion", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetApplyConversion",
                        params: &[Param { name: "aApplyConversion", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void doApplyContentConversions (in nsIStreamListener aNextListener, out nsIStreamListener aNewNextListener, in nsISupports aCtxt); */
                    Method {
                        name: "DoApplyContentConversions",
                        params: &[Param { name: "aNextListener", ty: "*const nsIStreamListener" }, Param { name: "aNewNextListener", ty: "*mut*const nsIStreamListener" }, Param { name: "aCtxt", ty: "*const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

