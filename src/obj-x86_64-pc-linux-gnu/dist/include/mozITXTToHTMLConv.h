/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/streamconv/mozITXTToHTMLConv.idl
 */

#ifndef __gen_mozITXTToHTMLConv_h__
#define __gen_mozITXTToHTMLConv_h__


#ifndef __gen_nsIStreamConverter_h__
#include "nsIStreamConverter.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
// {77c0e42a-1dd2-11b2-8ebf-edc6606f2f4b}
#define MOZITXTTOHTMLCONV_CID \
    { 0x77c0e42a, 0x1dd2, 0x11b2, \
    { 0x8e, 0xbf, 0xed, 0xc6, 0x60, 0x6f, 0x2f, 0x4b } }
#define MOZ_TXTTOHTMLCONV_CONTRACTID \
  "@mozilla.org/txttohtmlconv;1"

/* starting interface:    mozITXTToHTMLConv */
#define MOZITXTTOHTMLCONV_IID_STR "77c0e42a-1dd2-11b2-8ebf-edc6606f2f4b"

#define MOZITXTTOHTMLCONV_IID \
  {0x77c0e42a, 0x1dd2, 0x11b2, \
    { 0x8e, 0xbf, 0xed, 0xc6, 0x60, 0x6f, 0x2f, 0x4b }}

class NS_NO_VTABLE mozITXTToHTMLConv : public nsIStreamConverter {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZITXTTOHTMLCONV_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozITXTToHTMLConv;

  enum {
    kEntities = 0U,
    kURLs = 2U,
    kGlyphSubstitution = 4U,
    kStructPhrase = 8U
  };

  /* AString scanTXT (in AString text, in unsigned long whattodo); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ScanTXT(const nsAString& text, uint32_t whattodo, nsAString& _retval) = 0;

  /* AString scanHTML (in AString text, in unsigned long whattodo); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ScanHTML(const nsAString& text, uint32_t whattodo, nsAString& _retval) = 0;

  /* unsigned long citeLevelTXT (in wstring line, out unsigned long logLineStart); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CiteLevelTXT(const char16_t * line, uint32_t *logLineStart, uint32_t *_retval) = 0;

  /* void findURLInPlaintext (in wstring text, in long aLength, in long aPos, out long aStartPos, out long aEndPos); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD FindURLInPlaintext(const char16_t * text, int32_t aLength, int32_t aPos, int32_t *aStartPos, int32_t *aEndPos) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozITXTToHTMLConv, MOZITXTTOHTMLCONV_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZITXTTOHTMLCONV \
  NS_IMETHOD ScanTXT(const nsAString& text, uint32_t whattodo, nsAString& _retval) override; \
  NS_IMETHOD ScanHTML(const nsAString& text, uint32_t whattodo, nsAString& _retval) override; \
  NS_IMETHOD CiteLevelTXT(const char16_t * line, uint32_t *logLineStart, uint32_t *_retval) override; \
  NS_IMETHOD FindURLInPlaintext(const char16_t * text, int32_t aLength, int32_t aPos, int32_t *aStartPos, int32_t *aEndPos) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZITXTTOHTMLCONV \
  nsresult ScanTXT(const nsAString& text, uint32_t whattodo, nsAString& _retval); \
  nsresult ScanHTML(const nsAString& text, uint32_t whattodo, nsAString& _retval); \
  nsresult CiteLevelTXT(const char16_t * line, uint32_t *logLineStart, uint32_t *_retval); \
  nsresult FindURLInPlaintext(const char16_t * text, int32_t aLength, int32_t aPos, int32_t *aStartPos, int32_t *aEndPos); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZITXTTOHTMLCONV(_to) \
  NS_IMETHOD ScanTXT(const nsAString& text, uint32_t whattodo, nsAString& _retval) override { return _to ScanTXT(text, whattodo, _retval); } \
  NS_IMETHOD ScanHTML(const nsAString& text, uint32_t whattodo, nsAString& _retval) override { return _to ScanHTML(text, whattodo, _retval); } \
  NS_IMETHOD CiteLevelTXT(const char16_t * line, uint32_t *logLineStart, uint32_t *_retval) override { return _to CiteLevelTXT(line, logLineStart, _retval); } \
  NS_IMETHOD FindURLInPlaintext(const char16_t * text, int32_t aLength, int32_t aPos, int32_t *aStartPos, int32_t *aEndPos) override { return _to FindURLInPlaintext(text, aLength, aPos, aStartPos, aEndPos); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZITXTTOHTMLCONV(_to) \
  NS_IMETHOD ScanTXT(const nsAString& text, uint32_t whattodo, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ScanTXT(text, whattodo, _retval); } \
  NS_IMETHOD ScanHTML(const nsAString& text, uint32_t whattodo, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ScanHTML(text, whattodo, _retval); } \
  NS_IMETHOD CiteLevelTXT(const char16_t * line, uint32_t *logLineStart, uint32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CiteLevelTXT(line, logLineStart, _retval); } \
  NS_IMETHOD FindURLInPlaintext(const char16_t * text, int32_t aLength, int32_t aPos, int32_t *aStartPos, int32_t *aEndPos) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FindURLInPlaintext(text, aLength, aPos, aStartPos, aEndPos); } 


#endif /* __gen_mozITXTToHTMLConv_h__ */
