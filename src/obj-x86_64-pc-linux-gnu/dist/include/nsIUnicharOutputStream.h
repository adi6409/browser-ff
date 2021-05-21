/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIUnicharOutputStream.idl
 */

#ifndef __gen_nsIUnicharOutputStream_h__
#define __gen_nsIUnicharOutputStream_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIUnicharOutputStream */
#define NS_IUNICHAROUTPUTSTREAM_IID_STR "2d00b1bb-8b21-4a63-bcc6-7213f513ac2e"

#define NS_IUNICHAROUTPUTSTREAM_IID \
  {0x2d00b1bb, 0x8b21, 0x4a63, \
    { 0xbc, 0xc6, 0x72, 0x13, 0xf5, 0x13, 0xac, 0x2e }}

class NS_NO_VTABLE nsIUnicharOutputStream : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IUNICHAROUTPUTSTREAM_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIUnicharOutputStream;

  /* boolean write (in unsigned long aCount, [array, size_is (aCount), const] in char16_t c); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Write(uint32_t aCount, const char16_t *c, bool *_retval) = 0;

  /* boolean writeString (in AString str); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD WriteString(const nsAString& str, bool *_retval) = 0;

  /* void flush (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Flush(void) = 0;

  /* void close (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Close(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIUnicharOutputStream, NS_IUNICHAROUTPUTSTREAM_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIUNICHAROUTPUTSTREAM \
  NS_IMETHOD Write(uint32_t aCount, const char16_t *c, bool *_retval) override; \
  NS_IMETHOD WriteString(const nsAString& str, bool *_retval) override; \
  NS_IMETHOD Flush(void) override; \
  NS_IMETHOD Close(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIUNICHAROUTPUTSTREAM \
  nsresult Write(uint32_t aCount, const char16_t *c, bool *_retval); \
  nsresult WriteString(const nsAString& str, bool *_retval); \
  nsresult Flush(void); \
  nsresult Close(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIUNICHAROUTPUTSTREAM(_to) \
  NS_IMETHOD Write(uint32_t aCount, const char16_t *c, bool *_retval) override { return _to Write(aCount, c, _retval); } \
  NS_IMETHOD WriteString(const nsAString& str, bool *_retval) override { return _to WriteString(str, _retval); } \
  NS_IMETHOD Flush(void) override { return _to Flush(); } \
  NS_IMETHOD Close(void) override { return _to Close(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIUNICHAROUTPUTSTREAM(_to) \
  NS_IMETHOD Write(uint32_t aCount, const char16_t *c, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Write(aCount, c, _retval); } \
  NS_IMETHOD WriteString(const nsAString& str, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->WriteString(str, _retval); } \
  NS_IMETHOD Flush(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Flush(); } \
  NS_IMETHOD Close(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Close(); } 


#endif /* __gen_nsIUnicharOutputStream_h__ */
