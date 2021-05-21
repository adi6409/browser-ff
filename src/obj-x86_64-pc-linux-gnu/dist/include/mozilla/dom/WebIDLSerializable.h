#ifndef mozilla_dom_WebIDLSerializable_h
#define mozilla_dom_WebIDLSerializable_h

#include "js/TypeDecls.h"
#include "mozilla/dom/DOMJSClass.h"
#include "mozilla/dom/StructuredCloneTags.h"

namespace mozilla {
namespace dom {
WebIDLDeserializer LookupDeserializer(StructuredCloneTags aTag);
} // namespace dom
} // namespace mozilla


#endif // mozilla_dom_WebIDLSerializable_h
