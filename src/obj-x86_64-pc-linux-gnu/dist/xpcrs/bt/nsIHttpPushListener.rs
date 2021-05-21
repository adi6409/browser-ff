//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIHttpPushListener.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIHttpPushListener",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onPush (in nsIHttpChannel associatedChannel, in nsIHttpChannel pushChannel); */
                    Method {
                        name: "OnPush",
                        params: &[Param { name: "associatedChannel", ty: "*const nsIHttpChannel" }, Param { name: "pushChannel", ty: "*const nsIHttpChannel" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

