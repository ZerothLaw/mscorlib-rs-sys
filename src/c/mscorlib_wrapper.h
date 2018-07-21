#pragma once

#ifdef __cplusplus

#include <OAIdl.h>
#import "mscorlib.tlb" raw_interfaces_only no_smart_pointers \
    high_property_prefixes("_get","_put","_putref")		\
    rename("ReportEvent", "InteropServices_ReportEvent") \
	rename("or", "ORR")

#include <comdef.h>

using namespace mscorlib;
extern "C" {
#endif

#ifdef __cplusplus
}
#endif