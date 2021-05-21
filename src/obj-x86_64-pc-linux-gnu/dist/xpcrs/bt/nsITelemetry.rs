//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/telemetry/core/nsITelemetry.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIFetchTelemetryDataCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void complete (); */
                    Method {
                        name: "Complete",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsITelemetry",
            base: Some("nsISupports"),
            methods: Err("jscontext is unsupported"),
        },

        ]; D}

