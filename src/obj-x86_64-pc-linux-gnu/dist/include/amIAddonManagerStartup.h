/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/mozapps/extensions/amIAddonManagerStartup.idl
 */

#ifndef __gen_amIAddonManagerStartup_h__
#define __gen_amIAddonManagerStartup_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "js/Value.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIFile; /* forward declaration */

class nsIJSRAIIHelper; /* forward declaration */

class nsIURI; /* forward declaration */


/* starting interface:    amIAddonManagerStartup */
#define AMIADDONMANAGERSTARTUP_IID_STR "01dfa47b-87e4-4135-877b-586d033e1b5d"

#define AMIADDONMANAGERSTARTUP_IID \
  {0x01dfa47b, 0x87e4, 0x4135, \
    { 0x87, 0x7b, 0x58, 0x6d, 0x03, 0x3e, 0x1b, 0x5d }}

class NS_NO_VTABLE amIAddonManagerStartup : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(AMIADDONMANAGERSTARTUP_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = amIAddonManagerStartup;

  /* [implicit_jscontext] jsval readStartupData (); */
  NS_IMETHOD ReadStartupData(JSContext* cx, JS::MutableHandleValue _retval) = 0;

  /* [implicit_jscontext] nsIJSRAIIHelper registerChrome (in nsIURI manifestURI, in jsval entries); */
  NS_IMETHOD RegisterChrome(nsIURI *manifestURI, JS::HandleValue entries, JSContext* cx, nsIJSRAIIHelper **_retval) = 0;

  /* [implicit_jscontext] jsval encodeBlob (in jsval value); */
  NS_IMETHOD EncodeBlob(JS::HandleValue value, JSContext* cx, JS::MutableHandleValue _retval) = 0;

  /* [implicit_jscontext] jsval decodeBlob (in jsval value); */
  NS_IMETHOD DecodeBlob(JS::HandleValue value, JSContext* cx, JS::MutableHandleValue _retval) = 0;

  /* Array<AString> enumerateJAR (in nsIURI uri, in AUTF8String pattern); */
  NS_IMETHOD EnumerateJAR(nsIURI *uri, const nsACString& pattern, nsTArray<nsString >& _retval) = 0;

  /* Array<AString> enumerateJARSubtree (in nsIURI uri); */
  NS_IMETHOD EnumerateJARSubtree(nsIURI *uri, nsTArray<nsString >& _retval) = 0;

  /* void initializeURLPreloader (); */
  NS_IMETHOD InitializeURLPreloader(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(amIAddonManagerStartup, AMIADDONMANAGERSTARTUP_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_AMIADDONMANAGERSTARTUP \
  NS_IMETHOD ReadStartupData(JSContext* cx, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD RegisterChrome(nsIURI *manifestURI, JS::HandleValue entries, JSContext* cx, nsIJSRAIIHelper **_retval) override; \
  NS_IMETHOD EncodeBlob(JS::HandleValue value, JSContext* cx, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD DecodeBlob(JS::HandleValue value, JSContext* cx, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD EnumerateJAR(nsIURI *uri, const nsACString& pattern, nsTArray<nsString >& _retval) override; \
  NS_IMETHOD EnumerateJARSubtree(nsIURI *uri, nsTArray<nsString >& _retval) override; \
  NS_IMETHOD InitializeURLPreloader(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_AMIADDONMANAGERSTARTUP \
  nsresult ReadStartupData(JSContext* cx, JS::MutableHandleValue _retval); \
  nsresult RegisterChrome(nsIURI *manifestURI, JS::HandleValue entries, JSContext* cx, nsIJSRAIIHelper **_retval); \
  nsresult EncodeBlob(JS::HandleValue value, JSContext* cx, JS::MutableHandleValue _retval); \
  nsresult DecodeBlob(JS::HandleValue value, JSContext* cx, JS::MutableHandleValue _retval); \
  nsresult EnumerateJAR(nsIURI *uri, const nsACString& pattern, nsTArray<nsString >& _retval); \
  nsresult EnumerateJARSubtree(nsIURI *uri, nsTArray<nsString >& _retval); \
  nsresult InitializeURLPreloader(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_AMIADDONMANAGERSTARTUP(_to) \
  NS_IMETHOD ReadStartupData(JSContext* cx, JS::MutableHandleValue _retval) override { return _to ReadStartupData(cx, _retval); } \
  NS_IMETHOD RegisterChrome(nsIURI *manifestURI, JS::HandleValue entries, JSContext* cx, nsIJSRAIIHelper **_retval) override { return _to RegisterChrome(manifestURI, entries, cx, _retval); } \
  NS_IMETHOD EncodeBlob(JS::HandleValue value, JSContext* cx, JS::MutableHandleValue _retval) override { return _to EncodeBlob(value, cx, _retval); } \
  NS_IMETHOD DecodeBlob(JS::HandleValue value, JSContext* cx, JS::MutableHandleValue _retval) override { return _to DecodeBlob(value, cx, _retval); } \
  NS_IMETHOD EnumerateJAR(nsIURI *uri, const nsACString& pattern, nsTArray<nsString >& _retval) override { return _to EnumerateJAR(uri, pattern, _retval); } \
  NS_IMETHOD EnumerateJARSubtree(nsIURI *uri, nsTArray<nsString >& _retval) override { return _to EnumerateJARSubtree(uri, _retval); } \
  NS_IMETHOD InitializeURLPreloader(void) override { return _to InitializeURLPreloader(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_AMIADDONMANAGERSTARTUP(_to) \
  NS_IMETHOD ReadStartupData(JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ReadStartupData(cx, _retval); } \
  NS_IMETHOD RegisterChrome(nsIURI *manifestURI, JS::HandleValue entries, JSContext* cx, nsIJSRAIIHelper **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RegisterChrome(manifestURI, entries, cx, _retval); } \
  NS_IMETHOD EncodeBlob(JS::HandleValue value, JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EncodeBlob(value, cx, _retval); } \
  NS_IMETHOD DecodeBlob(JS::HandleValue value, JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DecodeBlob(value, cx, _retval); } \
  NS_IMETHOD EnumerateJAR(nsIURI *uri, const nsACString& pattern, nsTArray<nsString >& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EnumerateJAR(uri, pattern, _retval); } \
  NS_IMETHOD EnumerateJARSubtree(nsIURI *uri, nsTArray<nsString >& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EnumerateJARSubtree(uri, _retval); } \
  NS_IMETHOD InitializeURLPreloader(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitializeURLPreloader(); } 


#endif /* __gen_amIAddonManagerStartup_h__ */
