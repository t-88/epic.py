class DotDict(dict):
    __getattr__ = dict.get
