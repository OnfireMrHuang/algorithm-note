# 哈希表

哈希表是一种非常重要的数据结构，，它的特点是能够以 O(1) 的时间复杂度进行插入、删除和查找操作。

**实现原理**:

哈希表的实现原理是通过哈希函数将键映射到一个大数组中的一个位置来访问记录，然后通过数组位置索引快速的找到记录，
其中，每个位置又可以被称为槽（slot）或者桶（bucket）。所以哈希表仅适用于点查操作，而不适用于范围查找。

通过分析哈希表的实现原理，我们可以归纳出一些问题，比如：

- 哈希冲突问题: 不同的key通过哈希函数映射到了同一个位置，这个时候发生冲突，要如何解决?
- 散列不均衡问题: 如果哈希算法设计不合理，导致不同的key映射到了同一个位置的概率过大，这个时候就会导致哈希表中的元素分布不均衡，加剧冲突和性能下降,要如何解决?
- 扩缩容问题: 当哈希表中的元素越来越多时，哈希表的装载因子越来越大，这个时候哈希表的冲突就会进一步加剧以及性能下降，这个时候就需要扩容，扩容的过程又是怎样的呢？同理，当哈希表中的元素越来越少时，哈希表的装载因子越来越小，这个时候哈希表的空间利用率就会降低，这个时候就需要缩容，缩容的过程又是怎样的呢？

下面我们就来进一步了解哈希表针对这些问题的解决方案。

## 哈希算法

针对哈希算法的设计，我们往往希望其具有以下特点：

- 确定性：对于相同的输入，哈希算法应始终产生相同的输出。这样才能确保哈希表是可靠的。
- 效率高：计算哈希值的过程应该足够快。计算开销越小，哈希表的实用性越高。
- 均匀分布：哈希算法应使得键值对均匀分布在哈希表中。分布越均匀，哈希冲突的概率就越低

在实际应用中，我们常常会会根据不同的应用场景来设计哈希算法，比如:

- 对于散列程度足够高的key值，我们仅对其取模运算等简单的运算即可，比如身份证号。
- 对于散列程度不高的key值，我们可以通过一些复杂的运算来提高散列程度，比如使用常见的MD5、SHA、CRC等等。
- 同时我们还可以根据key的类型来选择不同的哈希算法，比如整型我们可以使用直接寻址法，字符串我们可以使用BKDRHash、APHash等等。

## 哈希冲突

通常情况下哈希函数的输入空间远大于输出空间，因此理论上哈希冲突是不可避免的。比如，输入空间为全体整数，输出空间为数组容量大小，则必然有多个整数映射至同一桶索引。

哈希冲突会导致查询结果错误，严重影响哈希表的可用性。为解决该问题，我们可以每当遇到哈希冲突就进行哈希表扩容，直至冲突消失为止。此方法简单粗暴且有效，但效率太低，因为哈希表扩容需要进行大量的数据搬运与哈希值计算。为了提升效率，我们仅在必要时，即当哈希冲突比较严重时，才执行扩容操作。目前哈希表解决哈希冲突的方法主要有两种：链式地址法和开放寻址法。

关于链式地址法和开放寻址法更具体的描述，可以参考[这里](https://www.hello-algo.com/chapter_hashing/hash_collision/#621)

## 哈希表的扩缩容

以Redis的hashmap为例子，其实现原理如下:

### 扩容

扩容其实就是一般是在 add 元素的时候校验一下是否达到某个阈值，然后决定要不要进行扩容。所以经过搜索可以看到添加元素会调用 dictAddRaw 这个函数，我们通过函数的注释也可以知道它是 add 或查找的底层的函数。

> Low level add or find:
>
> This function adds the entry but instead of setting a value returns the dictEntry structure to the user, that will make sure to fill the value field as he wishes.

dicAddRaw 函数会调用到 _dictKeyIndex 函数，这个函数会调用_dictExpandIfNeeded 判断是否需要扩容。

_dictExpandIfNeeded 函数判断了大致有三种情况会进行扩容：

- 如果 hash 表的size为0，那么创建一个容量为4的hash表；
- 服务器目前没有在执行 rdb 或者 aof 操作， 并且哈希表的负载因子大于等于 1；
- 服务器目前正在执行 rdb 或者 aof 操作， 并且哈希表的负载因子大于等于 5 ；

其中哈希表的负载因子可以通过公式：

```sh
// load ratio = the number of elements / the buckets
load_ratio = ht[0].used / ht[0].size
```

比如说， 对于一个大小为 4 ， 包含 4 个键值对的哈希表来说， 这个哈希表的负载因子为：

```sh
load_ratio = 4 / 4 = 1
```

又比如说， 对于一个大小为 512 ， 包含 256 个键值对的哈希表来说， 这个哈希表的负载因子为：

```sh
load_ratio = 256 / 512 = 0.5
```

为什么要根据 rdb 或者 aof 操作联合负载因子来判断是否应该扩容呢？其实源码的注释中也有提到：
> as we use copy-on-write and don’t want to move too much memory around when there is a child performing saving operations.

也就是说在 copy-on-write 时提高执行扩展操作所需的负载因子， 可以尽可能地避免在子进程存在期间进行哈希表扩展操作， 这可以避免不必要的内存写入操作， 最大限度地节约内存，提高子进程的操作的性能。

源码如下:

```c
static int _dictExpandIfNeeded(dict *d)
{ 
    // 正在扩容中
    if (dictIsRehashing(d)) return DICT_OK; 
    // 如果 hash 表的size为0，那么创建一个容量为4的hash表
    if (d->ht[0].size == 0) return dictExpand(d, DICT_HT_INITIAL_SIZE);

    // hash表中元素的个数已经大于hash表桶的数量
    if (d->ht[0].used >= d->ht[0].size &&
        //dict_can_resize 表示是否可以扩容
        (dict_can_resize ||
        // hash表中元素的个数已经除以hash表桶的数量是否大于5
         d->ht[0].used/d->ht[0].size > dict_force_resize_ratio))
    {
        return dictExpand(d, d->ht[0].used*2); // 容量扩大两倍
    }
    return DICT_OK;
}

int dictExpand(dict *d, unsigned long size)
{
    //正在扩容，直接返回
    if (dictIsRehashing(d) || d->ht[0].used > size)
        return DICT_ERR;

    dictht n;  
    // _dictNextPower会返回 size 最接近的2的指数值
    // 也就是size是10，那么返回 16，size是20，那么返回32
    unsigned long realsize = _dictNextPower(size); 

    // 校验扩容之后的值是否和当前一样
    if (realsize == d->ht[0].size) return DICT_ERR;
    // 初始化 dictht 成员变量
    n.size = realsize;
    n.sizemask = realsize-1;
    n.table = zcalloc(realsize*sizeof(dictEntry*)); // 申请空间是 size * Entry的大小
    n.used = 0;

    //校验hash 表是否初始化过，没有初始化不应该进行rehash
    if (d->ht[0].table == NULL) {
        d->ht[0] = n;
        return DICT_OK;
    }
    //将新的hash表赋值给 ht[1]
    d->ht[1] = n;
    d->rehashidx = 0;
    return DICT_OK;
}
```

### 缩容

在 Redis 里面对于清理过期数据一个是惰性删除，另一个是定期删除，缩容其实也是在定期删除里面做的。

Redis 的定时器会每100ms调用一次 databasesCron 函数，它会调用到 dictResize 函数进行缩容：

同样的 dictResize 函数中也会判断一下是否正在执行 rehash 以及校验 dict_can_resize 是否在进行 copy on write操作。然后将 hash 表的 bucket 大小缩小为和被键值对同样大小：

```c
int dictResize(dict *d)
{
    int minimal;

    if (!dict_can_resize || dictIsRehashing(d)) return DICT_ERR;
    minimal = d->ht[0].used; // 将bucket 缩小为和被键值对同样大小
    if (minimal < DICT_HT_INITIAL_SIZE)
        minimal = DICT_HT_INITIAL_SIZE;
    return dictExpand(d, minimal);
}
```
