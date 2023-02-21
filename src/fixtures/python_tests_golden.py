from collections import OrderedDict
from typing import Any
num_str = some_num.__str__() 
num_repr = some_num.__add__(2) 
my_repr = my_module.my_object.__repr__() 
MY_CONTAINS_BAD = {1, 2, 3}.__contains__(1) 
MY_CONTAINS_GOOD = 1 in {1, 2, 3}
my_list_bad = []
my_list_bad.__init__({1, 2, 3}) 
my_list_good = list({1, 2, 3})
my_woohoo = my_object.__woohoo__()
def is_bigger_than_two(val):
    return val.__gt__(2)  
class Foo1:
    def __init__(self):
        object.__init__(self)
class Foo2:
    def __init__(self):
        super().__init__(self)
class Bar1:
    def __new__(cls):
        object.__new__(cls)
class Bar2:
    def __new__(cls):
        super().__new__(cls)
class CustomRegistry(dict):
    def __init__(self) -> None:
        super().__init__()
        self._entry_ids = {}
    def __setitem__(self, key, entry) -> None:
        super().__setitem__(key, entry)
        self._entry_ids.__setitem__(entry.id, entry)
        self._entry_ids.__delitem__(entry.id)
    def __delitem__(self, key: str) -> None:
        entry = self[key]
        self._entry_ids.__delitem__(entry.id)
        super().__delitem__(key)
class CustomState:
    def __init__(self, state):
        self._state = state
    def __eq__(self, other: Any) -> bool:
        return self._state.__eq__(other)
class CustomDict(OrderedDict):
    def __init__(self, *args, **kwds):
        OrderedDict.__init__(self, *args, **kwds)
    def __setitem__(self, key, value):
        OrderedDict.__setitem__(self, key, value)
class MyClass(list):
    def __contains__(self, item):
        print()
        return super().__contains__(item)
class PluginBase:
    subclasses = []
    def __init_subclass__(cls, **kwargs):
        super().__init_subclass__(**kwargs)
        cls.subclasses.append(cls)
class SomeClass:
    def __init__(self):
        self.my_attr = object()
    def __setattr__(self, name, value):
        def nested_function():
            self.my_attr.__setattr__(name, value)
        nested_function()
class Base:
    @classmethod
    def get_first_subclass(cls):
        for subklass in cls.__subclasses__():
            return subklass
        return object
my_instance_name = x.__class__.__name__
my_pkg_version = pkg.__version__
MANUAL_SELF = int.__add__(1, 1)
MY_DICT = {: 1, : 2}
dict.__setitem__(MY_DICT, , )
INSTANTIATED_SELF = int().__add__(1) 
{: 1, : 2}.__setitem__(, ) 
a = [1, 2, 3]
assert super(type(a), a).__str__() == 
class MyString(str):
    def rjust(self, width, fillchar= ):
        width = width.__index__()