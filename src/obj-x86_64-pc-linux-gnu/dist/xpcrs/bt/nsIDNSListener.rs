//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/dns/nsIDNSListener.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDNSListener",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onLookupComplete (in nsICancelable aRequest, in nsIDNSRecord aRecord, in nsresult aStatus); */
                    Method {
                        name: "OnLookupComplete",
                        params: &[Param { name: "aRequest", ty: "*const nsICancelable" }, Param { name: "aRecord", ty: "*const nsIDNSRecord" }, Param { name: "aStatus", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

