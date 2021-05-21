//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/storage/mozIStorageCompletionCallback.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "mozIStorageCompletionCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void complete (in nsresult status, [optional] in nsISupports value); */
                    Method {
                        name: "Complete",
                        params: &[Param { name: "status", ty: "::nserror::nsresult" }, Param { name: "value", ty: "*const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

