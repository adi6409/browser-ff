//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsITraceableChannel.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsITraceableChannel",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* nsIStreamListener setNewListener (in nsIStreamListener aListener, [optional] in boolean aMustApplyContentConversion); */
                    Method {
                        name: "SetNewListener",
                        params: &[Param { name: "aListener", ty: "*const nsIStreamListener" }, Param { name: "aMustApplyContentConversion", ty: "bool" }, Param { name: "_retval", ty: "*mut*const nsIStreamListener" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

