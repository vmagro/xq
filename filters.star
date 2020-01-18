def attr(name):
    return lambda e: e.attrs[name]

def children(tag):
    return lambda e: [c for c in e.children if e.tag == tag]