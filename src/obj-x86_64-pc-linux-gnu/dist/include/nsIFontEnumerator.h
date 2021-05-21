/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/gfx/src/nsIFontEnumerator.idl
 */

#ifndef __gen_nsIFontEnumerator_h__
#define __gen_nsIFontEnumerator_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "js/Value.h"

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIFontEnumerator */
#define NS_IFONTENUMERATOR_IID_STR "924d98d9-3518-4cb4-8708-c74fe8e3ec3c"

#define NS_IFONTENUMERATOR_IID \
  {0x924d98d9, 0x3518, 0x4cb4, \
    { 0x87, 0x08, 0xc7, 0x4f, 0xe8, 0xe3, 0xec, 0x3c }}

class NS_NO_VTABLE nsIFontEnumerator : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IFONTENUMERATOR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIFontEnumerator;

  /* Array<AString> EnumerateAllFonts (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD EnumerateAllFonts(nsTArray<nsString >& _retval) = 0;

  /* Array<AString> EnumerateFonts (in string aLangGroup, in string aGeneric); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD EnumerateFonts(const char * aLangGroup, const char * aGeneric, nsTArray<nsString >& _retval) = 0;

  /* [implicit_jscontext] jsval EnumerateAllFontsAsync (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD EnumerateAllFontsAsync(JSContext* cx, JS::MutableHandleValue _retval) = 0;

  /* [implicit_jscontext] jsval EnumerateFontsAsync (in string aLangGroup, in string aGeneric); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD EnumerateFontsAsync(const char * aLangGroup, const char * aGeneric, JSContext* cx, JS::MutableHandleValue _retval) = 0;

  /* void HaveFontFor (in string aLangGroup, [retval] out boolean aResult); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HaveFontFor(const char * aLangGroup, bool *aResult) = 0;

  /* wstring getDefaultFont (in string aLangGroup, in string aGeneric); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDefaultFont(const char * aLangGroup, const char * aGeneric, char16_t * *_retval) = 0;

  /* boolean updateFontList (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD UpdateFontList(bool *_retval) = 0;

  /* wstring getStandardFamilyName (in wstring aName); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetStandardFamilyName(const char16_t * aName, char16_t * *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIFontEnumerator, NS_IFONTENUMERATOR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIFONTENUMERATOR \
  NS_IMETHOD EnumerateAllFonts(nsTArray<nsString >& _retval) override; \
  NS_IMETHOD EnumerateFonts(const char * aLangGroup, const char * aGeneric, nsTArray<nsString >& _retval) override; \
  NS_IMETHOD EnumerateAllFontsAsync(JSContext* cx, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD EnumerateFontsAsync(const char * aLangGroup, const char * aGeneric, JSContext* cx, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD HaveFontFor(const char * aLangGroup, bool *aResult) override; \
  NS_IMETHOD GetDefaultFont(const char * aLangGroup, const char * aGeneric, char16_t * *_retval) override; \
  NS_IMETHOD UpdateFontList(bool *_retval) override; \
  NS_IMETHOD GetStandardFamilyName(const char16_t * aName, char16_t * *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIFONTENUMERATOR \
  nsresult EnumerateAllFonts(nsTArray<nsString >& _retval); \
  nsresult EnumerateFonts(const char * aLangGroup, const char * aGeneric, nsTArray<nsString >& _retval); \
  nsresult EnumerateAllFontsAsync(JSContext* cx, JS::MutableHandleValue _retval); \
  nsresult EnumerateFontsAsync(const char * aLangGroup, const char * aGeneric, JSContext* cx, JS::MutableHandleValue _retval); \
  nsresult HaveFontFor(const char * aLangGroup, bool *aResult); \
  nsresult GetDefaultFont(const char * aLangGroup, const char * aGeneric, char16_t * *_retval); \
  nsresult UpdateFontList(bool *_retval); \
  nsresult GetStandardFamilyName(const char16_t * aName, char16_t * *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIFONTENUMERATOR(_to) \
  NS_IMETHOD EnumerateAllFonts(nsTArray<nsString >& _retval) override { return _to EnumerateAllFonts(_retval); } \
  NS_IMETHOD EnumerateFonts(const char * aLangGroup, const char * aGeneric, nsTArray<nsString >& _retval) override { return _to EnumerateFonts(aLangGroup, aGeneric, _retval); } \
  NS_IMETHOD EnumerateAllFontsAsync(JSContext* cx, JS::MutableHandleValue _retval) override { return _to EnumerateAllFontsAsync(cx, _retval); } \
  NS_IMETHOD EnumerateFontsAsync(const char * aLangGroup, const char * aGeneric, JSContext* cx, JS::MutableHandleValue _retval) override { return _to EnumerateFontsAsync(aLangGroup, aGeneric, cx, _retval); } \
  NS_IMETHOD HaveFontFor(const char * aLangGroup, bool *aResult) override { return _to HaveFontFor(aLangGroup, aResult); } \
  NS_IMETHOD GetDefaultFont(const char * aLangGroup, const char * aGeneric, char16_t * *_retval) override { return _to GetDefaultFont(aLangGroup, aGeneric, _retval); } \
  NS_IMETHOD UpdateFontList(bool *_retval) override { return _to UpdateFontList(_retval); } \
  NS_IMETHOD GetStandardFamilyName(const char16_t * aName, char16_t * *_retval) override { return _to GetStandardFamilyName(aName, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIFONTENUMERATOR(_to) \
  NS_IMETHOD EnumerateAllFonts(nsTArray<nsString >& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EnumerateAllFonts(_retval); } \
  NS_IMETHOD EnumerateFonts(const char * aLangGroup, const char * aGeneric, nsTArray<nsString >& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EnumerateFonts(aLangGroup, aGeneric, _retval); } \
  NS_IMETHOD EnumerateAllFontsAsync(JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EnumerateAllFontsAsync(cx, _retval); } \
  NS_IMETHOD EnumerateFontsAsync(const char * aLangGroup, const char * aGeneric, JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EnumerateFontsAsync(aLangGroup, aGeneric, cx, _retval); } \
  NS_IMETHOD HaveFontFor(const char * aLangGroup, bool *aResult) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HaveFontFor(aLangGroup, aResult); } \
  NS_IMETHOD GetDefaultFont(const char * aLangGroup, const char * aGeneric, char16_t * *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDefaultFont(aLangGroup, aGeneric, _retval); } \
  NS_IMETHOD UpdateFontList(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UpdateFontList(_retval); } \
  NS_IMETHOD GetStandardFamilyName(const char16_t * aName, char16_t * *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetStandardFamilyName(aName, _retval); } 


#endif /* __gen_nsIFontEnumerator_h__ */
