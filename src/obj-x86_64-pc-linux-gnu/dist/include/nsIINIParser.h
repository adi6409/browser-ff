/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/ds/nsIINIParser.idl
 */

#ifndef __gen_nsIINIParser_h__
#define __gen_nsIINIParser_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIUTF8StringEnumerator; /* forward declaration */

class nsIFile; /* forward declaration */


/* starting interface:    nsIINIParser */
#define NS_IINIPARSER_IID_STR "7eb955f6-3e78-4d39-b72f-c1bf12a94bce"

#define NS_IINIPARSER_IID \
  {0x7eb955f6, 0x3e78, 0x4d39, \
    { 0xb7, 0x2f, 0xc1, 0xbf, 0x12, 0xa9, 0x4b, 0xce }}

class NS_NO_VTABLE nsIINIParser : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IINIPARSER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIINIParser;

  /* nsIUTF8StringEnumerator getSections (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSections(nsIUTF8StringEnumerator **_retval) = 0;

  /* nsIUTF8StringEnumerator getKeys (in AUTF8String aSection); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetKeys(const nsACString& aSection, nsIUTF8StringEnumerator **_retval) = 0;

  /* AUTF8String getString (in AUTF8String aSection, in AUTF8String aKey); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetString(const nsACString& aSection, const nsACString& aKey, nsACString& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIINIParser, NS_IINIPARSER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIINIPARSER \
  NS_IMETHOD GetSections(nsIUTF8StringEnumerator **_retval) override; \
  NS_IMETHOD GetKeys(const nsACString& aSection, nsIUTF8StringEnumerator **_retval) override; \
  NS_IMETHOD GetString(const nsACString& aSection, const nsACString& aKey, nsACString& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIINIPARSER \
  nsresult GetSections(nsIUTF8StringEnumerator **_retval); \
  nsresult GetKeys(const nsACString& aSection, nsIUTF8StringEnumerator **_retval); \
  nsresult GetString(const nsACString& aSection, const nsACString& aKey, nsACString& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIINIPARSER(_to) \
  NS_IMETHOD GetSections(nsIUTF8StringEnumerator **_retval) override { return _to GetSections(_retval); } \
  NS_IMETHOD GetKeys(const nsACString& aSection, nsIUTF8StringEnumerator **_retval) override { return _to GetKeys(aSection, _retval); } \
  NS_IMETHOD GetString(const nsACString& aSection, const nsACString& aKey, nsACString& _retval) override { return _to GetString(aSection, aKey, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIINIPARSER(_to) \
  NS_IMETHOD GetSections(nsIUTF8StringEnumerator **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSections(_retval); } \
  NS_IMETHOD GetKeys(const nsACString& aSection, nsIUTF8StringEnumerator **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetKeys(aSection, _retval); } \
  NS_IMETHOD GetString(const nsACString& aSection, const nsACString& aKey, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetString(aSection, aKey, _retval); } 


/* starting interface:    nsIINIParserWriter */
#define NS_IINIPARSERWRITER_IID_STR "b67bb24b-31a3-4a6a-a5d9-0485c9af5a04"

#define NS_IINIPARSERWRITER_IID \
  {0xb67bb24b, 0x31a3, 0x4a6a, \
    { 0xa5, 0xd9, 0x04, 0x85, 0xc9, 0xaf, 0x5a, 0x04 }}

class NS_NO_VTABLE nsIINIParserWriter : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IINIPARSERWRITER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIINIParserWriter;

  /* void setString (in AUTF8String aSection, in AUTF8String aKey, in AUTF8String aValue); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetString(const nsACString& aSection, const nsACString& aKey, const nsACString& aValue) = 0;

  /* void writeFile (in nsIFile aINIFile); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD WriteFile(nsIFile *aINIFile) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIINIParserWriter, NS_IINIPARSERWRITER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIINIPARSERWRITER \
  NS_IMETHOD SetString(const nsACString& aSection, const nsACString& aKey, const nsACString& aValue) override; \
  NS_IMETHOD WriteFile(nsIFile *aINIFile) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIINIPARSERWRITER \
  nsresult SetString(const nsACString& aSection, const nsACString& aKey, const nsACString& aValue); \
  nsresult WriteFile(nsIFile *aINIFile); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIINIPARSERWRITER(_to) \
  NS_IMETHOD SetString(const nsACString& aSection, const nsACString& aKey, const nsACString& aValue) override { return _to SetString(aSection, aKey, aValue); } \
  NS_IMETHOD WriteFile(nsIFile *aINIFile) override { return _to WriteFile(aINIFile); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIINIPARSERWRITER(_to) \
  NS_IMETHOD SetString(const nsACString& aSection, const nsACString& aKey, const nsACString& aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetString(aSection, aKey, aValue); } \
  NS_IMETHOD WriteFile(nsIFile *aINIFile) override { return !_to ? NS_ERROR_NULL_POINTER : _to->WriteFile(aINIFile); } 


/* starting interface:    nsIINIParserFactory */
#define NS_IINIPARSERFACTORY_IID_STR "ccae7ea5-1218-4b51-aecb-c2d8ecd46af9"

#define NS_IINIPARSERFACTORY_IID \
  {0xccae7ea5, 0x1218, 0x4b51, \
    { 0xae, 0xcb, 0xc2, 0xd8, 0xec, 0xd4, 0x6a, 0xf9 }}

class NS_NO_VTABLE nsIINIParserFactory : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IINIPARSERFACTORY_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIINIParserFactory;

  /* nsIINIParser createINIParser ([optional] in nsIFile aINIFile); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CreateINIParser(nsIFile *aINIFile, nsIINIParser **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIINIParserFactory, NS_IINIPARSERFACTORY_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIINIPARSERFACTORY \
  NS_IMETHOD CreateINIParser(nsIFile *aINIFile, nsIINIParser **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIINIPARSERFACTORY \
  nsresult CreateINIParser(nsIFile *aINIFile, nsIINIParser **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIINIPARSERFACTORY(_to) \
  NS_IMETHOD CreateINIParser(nsIFile *aINIFile, nsIINIParser **_retval) override { return _to CreateINIParser(aINIFile, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIINIPARSERFACTORY(_to) \
  NS_IMETHOD CreateINIParser(nsIFile *aINIFile, nsIINIParser **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateINIParser(aINIFile, _retval); } 


#endif /* __gen_nsIINIParser_h__ */
