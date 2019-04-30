TO_IGNORE = [
    'initGEOS_r', # deprecated in 3.5
    'finishGEOS_r',  # deprecated in 3.5
    'GEOSGeomFromWKT', # deprecated
    'GEOSGeomFromWKT_r', # deprecated 
    'GEOSGeomToWKT', # deprecated
    'GEOSGeomToWKT_r', # deprecated
    'GEOSSingleSidedBuffer', # deprecated in 3.3
    'GEOSSingleSidedBuffer_r', # deprecated in 3.3
    'GEOSUnionCascaded', # deprecated in 3.3
    'GEOSUnionCascaded_r', # deprecated in 3.3
]

def get_c_functions(filename):
    content = None
    functions = set()
    with open(filename) as f:
        content = f.read().splitlines()
    if content is None:
        return functions
    for line in content:
        if ' GEOS_DLL ' not in line:
            continue
        l = line.split(' GEOS_DLL ')[1].split('(')[0]
        while l.startswith('*'):
            l = l[1:]
        l = l.strip()
        if len(l) > 0 and l not in TO_IGNORE:
            functions.add(l)
    return functions

def get_rust_functions(filename, c_func):
    content = None
    functions = set()
    with open(filename) as f:
        content = f.read().splitlines()
    if content is None:
        return
    for line in content:
        if ' fn ' not in line:
            continue
        l = line.split(' fn ')[1].split('(')[0].strip()
        if l in c_func:
            c_func.remove(l)
            continue
        if len(l) > 0:
            functions.add(l)
    return functions

c_func = get_c_functions('check_missing/geos_c.h')
r_func = get_rust_functions('src/functions.rs', c_func)

x = []
print('==> Not bound functions:')
for f in c_func:
    x.append(f)
x.sort()
for f in x:
    print(f)
print('')
print('==> Extra (???) rust functions:')
x = []
for f in r_func:
    x.append(f)
x.sort()
for f in x:
    print(f)
