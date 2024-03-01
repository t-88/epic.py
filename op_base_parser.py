
class BaseParser:
    def __init__(self):
        self.idx = 0
        self.src = None
        
    def cur(self,offset = 0):
        if self.idx + offset >= len(self.src):
            print(f"[Out Of Range] {self.__class__} char out of range idx='{self.idx}'")
            exit(69)
        return self.src[self.idx + offset]

    def next(self):
        cur =  self.cur()
        self.idx += 1
        return cur        
