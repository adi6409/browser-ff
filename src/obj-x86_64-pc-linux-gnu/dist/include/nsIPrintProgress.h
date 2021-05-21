/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/layout/printing/nsIPrintProgress.idl
 */

#ifndef __gen_nsIPrintProgress_h__
#define __gen_nsIPrintProgress_h__


#ifndef __gen_nsIWebProgressListener_h__
#include "nsIWebProgressListener.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class mozIDOMWindowProxy; /* forward declaration */

class nsIObserver; /* forward declaration */

class nsIPrompt; /* forward declaration */


/* starting interface:    nsIPrintProgress */
#define NS_IPRINTPROGRESS_IID_STR "05f4fb88-e568-4d35-b394-ce0aa3eea6fc"

#define NS_IPRINTPROGRESS_IID \
  {0x05f4fb88, 0xe568, 0x4d35, \
    { 0xb3, 0x94, 0xce, 0x0a, 0xa3, 0xee, 0xa6, 0xfc }}

class NS_NO_VTABLE nsIPrintProgress : public nsIWebProgressListener {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPRINTPROGRESS_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPrintProgress;

  /* void openProgressDialog (in mozIDOMWindowProxy parent, in string dialogURL, in nsISupports parameters, in nsIObserver openDialogObserver, out boolean notifyOnOpen); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OpenProgressDialog(mozIDOMWindowProxy *parent, const char * dialogURL, nsISupports *parameters, nsIObserver *openDialogObserver, bool *notifyOnOpen) = 0;

  /* void closeProgressDialog (in boolean forceClose); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CloseProgressDialog(bool forceClose) = 0;

  /* void registerListener (in nsIWebProgressListener listener); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RegisterListener(nsIWebProgressListener *listener) = 0;

  /* void unregisterListener (in nsIWebProgressListener listener); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD UnregisterListener(nsIWebProgressListener *listener) = 0;

  /* void doneIniting (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DoneIniting(void) = 0;

  /* nsIPrompt getPrompter (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPrompter(nsIPrompt **_retval) = 0;

  /* attribute boolean processCanceledByUser; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetProcessCanceledByUser(bool *aProcessCanceledByUser) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetProcessCanceledByUser(bool aProcessCanceledByUser) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPrintProgress, NS_IPRINTPROGRESS_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPRINTPROGRESS \
  NS_IMETHOD OpenProgressDialog(mozIDOMWindowProxy *parent, const char * dialogURL, nsISupports *parameters, nsIObserver *openDialogObserver, bool *notifyOnOpen) override; \
  NS_IMETHOD CloseProgressDialog(bool forceClose) override; \
  NS_IMETHOD RegisterListener(nsIWebProgressListener *listener) override; \
  NS_IMETHOD UnregisterListener(nsIWebProgressListener *listener) override; \
  NS_IMETHOD DoneIniting(void) override; \
  NS_IMETHOD GetPrompter(nsIPrompt **_retval) override; \
  NS_IMETHOD GetProcessCanceledByUser(bool *aProcessCanceledByUser) override; \
  NS_IMETHOD SetProcessCanceledByUser(bool aProcessCanceledByUser) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPRINTPROGRESS \
  nsresult OpenProgressDialog(mozIDOMWindowProxy *parent, const char * dialogURL, nsISupports *parameters, nsIObserver *openDialogObserver, bool *notifyOnOpen); \
  nsresult CloseProgressDialog(bool forceClose); \
  nsresult RegisterListener(nsIWebProgressListener *listener); \
  nsresult UnregisterListener(nsIWebProgressListener *listener); \
  nsresult DoneIniting(void); \
  nsresult GetPrompter(nsIPrompt **_retval); \
  nsresult GetProcessCanceledByUser(bool *aProcessCanceledByUser); \
  nsresult SetProcessCanceledByUser(bool aProcessCanceledByUser); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPRINTPROGRESS(_to) \
  NS_IMETHOD OpenProgressDialog(mozIDOMWindowProxy *parent, const char * dialogURL, nsISupports *parameters, nsIObserver *openDialogObserver, bool *notifyOnOpen) override { return _to OpenProgressDialog(parent, dialogURL, parameters, openDialogObserver, notifyOnOpen); } \
  NS_IMETHOD CloseProgressDialog(bool forceClose) override { return _to CloseProgressDialog(forceClose); } \
  NS_IMETHOD RegisterListener(nsIWebProgressListener *listener) override { return _to RegisterListener(listener); } \
  NS_IMETHOD UnregisterListener(nsIWebProgressListener *listener) override { return _to UnregisterListener(listener); } \
  NS_IMETHOD DoneIniting(void) override { return _to DoneIniting(); } \
  NS_IMETHOD GetPrompter(nsIPrompt **_retval) override { return _to GetPrompter(_retval); } \
  NS_IMETHOD GetProcessCanceledByUser(bool *aProcessCanceledByUser) override { return _to GetProcessCanceledByUser(aProcessCanceledByUser); } \
  NS_IMETHOD SetProcessCanceledByUser(bool aProcessCanceledByUser) override { return _to SetProcessCanceledByUser(aProcessCanceledByUser); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPRINTPROGRESS(_to) \
  NS_IMETHOD OpenProgressDialog(mozIDOMWindowProxy *parent, const char * dialogURL, nsISupports *parameters, nsIObserver *openDialogObserver, bool *notifyOnOpen) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OpenProgressDialog(parent, dialogURL, parameters, openDialogObserver, notifyOnOpen); } \
  NS_IMETHOD CloseProgressDialog(bool forceClose) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CloseProgressDialog(forceClose); } \
  NS_IMETHOD RegisterListener(nsIWebProgressListener *listener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RegisterListener(listener); } \
  NS_IMETHOD UnregisterListener(nsIWebProgressListener *listener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UnregisterListener(listener); } \
  NS_IMETHOD DoneIniting(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DoneIniting(); } \
  NS_IMETHOD GetPrompter(nsIPrompt **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrompter(_retval); } \
  NS_IMETHOD GetProcessCanceledByUser(bool *aProcessCanceledByUser) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetProcessCanceledByUser(aProcessCanceledByUser); } \
  NS_IMETHOD SetProcessCanceledByUser(bool aProcessCanceledByUser) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetProcessCanceledByUser(aProcessCanceledByUser); } 


#endif /* __gen_nsIPrintProgress_h__ */
