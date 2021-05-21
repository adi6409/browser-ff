/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/xslt/xslt/txIEXSLTFunctions.idl
 */

#ifndef __gen_txIEXSLTFunctions_h__
#define __gen_txIEXSLTFunctions_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
namespace mozilla {
namespace dom {
class Document; /* webidl Document */
} // namespace dom
} // namespace mozilla

namespace mozilla {
namespace dom {
class DocumentFragment; /* webidl DocumentFragment */
} // namespace dom
} // namespace mozilla


/* starting interface:    txIEXSLTFunctions */
#define TXIEXSLTFUNCTIONS_IID_STR "21b1cfa4-00ce-4cc1-bfc1-92af1d00e580"

#define TXIEXSLTFUNCTIONS_IID \
  {0x21b1cfa4, 0x00ce, 0x4cc1, \
    { 0xbf, 0xc1, 0x92, 0xaf, 0x1d, 0x00, 0xe5, 0x80 }}

class NS_NO_VTABLE txIEXSLTFunctions : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(TXIEXSLTFUNCTIONS_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = txIEXSLTFunctions;

  /* DocumentFragment match (in AString str, in AString regex, in AString flags, in Document doc); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Match(const nsAString& str, const nsAString& regex, const nsAString& flags, mozilla::dom::Document *doc, mozilla::dom::DocumentFragment **_retval) = 0;

  /* AString replace (in AString str, in AString regex, in AString flags, in AString replace); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Replace(const nsAString& str, const nsAString& regex, const nsAString& flags, const nsAString& replace, nsAString& _retval) = 0;

  /* boolean test (in AString str, in AString regex, in AString flags); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Test(const nsAString& str, const nsAString& regex, const nsAString& flags, bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(txIEXSLTFunctions, TXIEXSLTFUNCTIONS_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_TXIEXSLTFUNCTIONS \
  NS_IMETHOD Match(const nsAString& str, const nsAString& regex, const nsAString& flags, mozilla::dom::Document *doc, mozilla::dom::DocumentFragment **_retval) override; \
  NS_IMETHOD Replace(const nsAString& str, const nsAString& regex, const nsAString& flags, const nsAString& replace, nsAString& _retval) override; \
  NS_IMETHOD Test(const nsAString& str, const nsAString& regex, const nsAString& flags, bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_TXIEXSLTFUNCTIONS \
  nsresult Match(const nsAString& str, const nsAString& regex, const nsAString& flags, mozilla::dom::Document *doc, mozilla::dom::DocumentFragment **_retval); \
  nsresult Replace(const nsAString& str, const nsAString& regex, const nsAString& flags, const nsAString& replace, nsAString& _retval); \
  nsresult Test(const nsAString& str, const nsAString& regex, const nsAString& flags, bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_TXIEXSLTFUNCTIONS(_to) \
  NS_IMETHOD Match(const nsAString& str, const nsAString& regex, const nsAString& flags, mozilla::dom::Document *doc, mozilla::dom::DocumentFragment **_retval) override { return _to Match(str, regex, flags, doc, _retval); } \
  NS_IMETHOD Replace(const nsAString& str, const nsAString& regex, const nsAString& flags, const nsAString& replace, nsAString& _retval) override { return _to Replace(str, regex, flags, replace, _retval); } \
  NS_IMETHOD Test(const nsAString& str, const nsAString& regex, const nsAString& flags, bool *_retval) override { return _to Test(str, regex, flags, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_TXIEXSLTFUNCTIONS(_to) \
  NS_IMETHOD Match(const nsAString& str, const nsAString& regex, const nsAString& flags, mozilla::dom::Document *doc, mozilla::dom::DocumentFragment **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Match(str, regex, flags, doc, _retval); } \
  NS_IMETHOD Replace(const nsAString& str, const nsAString& regex, const nsAString& flags, const nsAString& replace, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Replace(str, regex, flags, replace, _retval); } \
  NS_IMETHOD Test(const nsAString& str, const nsAString& regex, const nsAString& flags, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Test(str, regex, flags, _retval); } 


#endif /* __gen_txIEXSLTFunctions_h__ */
