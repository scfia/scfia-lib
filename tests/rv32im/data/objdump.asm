
target/riscv32im-unknown-none-elf/release/simple_router_risc_v:     file format elf32-littleriscv


Disassembly of section .text:

00000004 <_ZN20simple_router_risc_v6virtio23VirtioMMIONetworkDevice19configure_virtqueue17h6e1b5c65dc542e08E.llvm.1659980649332766766>:
       4:	fd010113          	addi	sp,sp,-48
       8:	02112623          	sw	ra,44(sp)
       c:	02812423          	sw	s0,40(sp)
      10:	02912223          	sw	s1,36(sp)
      14:	03212023          	sw	s2,32(sp)
      18:	00062483          	lw	s1,0(a2)
      1c:	02b4a823          	sw	a1,48(s1)
      20:	0404a583          	lw	a1,64(s1)
      24:	06059063          	bnez	a1,84 <_ZN20simple_router_risc_v6virtio23VirtioMMIONetworkDevice19configure_virtqueue17h6e1b5c65dc542e08E.llvm.1659980649332766766+0x80>
      28:	00050413          	mv	s0,a0
      2c:	0344a503          	lw	a0,52(s1)
      30:	08050463          	beqz	a0,b8 <_ZN20simple_router_risc_v6virtio23VirtioMMIONetworkDevice19configure_virtqueue17h6e1b5c65dc542e08E.llvm.1659980649332766766+0xb4>
      34:	40000913          	li	s2,1024
      38:	0b256a63          	bltu	a0,s2,ec <_ZN20simple_router_risc_v6virtio23VirtioMMIONetworkDevice19configure_virtqueue17h6e1b5c65dc542e08E.llvm.1659980649332766766+0xe8>
      3c:	40000593          	li	a1,1024
      40:	00040513          	mv	a0,s0
      44:	00068613          	mv	a2,a3
      48:	00070693          	mv	a3,a4
      4c:	00000097          	auipc	ra,0x0
      50:	14c080e7          	jalr	332(ra) # 198 <_ZN20simple_router_risc_v9virtqueue15VirtQueueHandle3new17h4cfd822ae4a1d98aE>
      54:	0324ac23          	sw	s2,56(s1)
      58:	00001537          	lui	a0,0x1
      5c:	02a4ae23          	sw	a0,60(s1)
      60:	00042503          	lw	a0,0(s0)
      64:	00b55513          	srli	a0,a0,0xb
      68:	04a4a023          	sw	a0,64(s1)
      6c:	02c12083          	lw	ra,44(sp)
      70:	02812403          	lw	s0,40(sp)
      74:	02412483          	lw	s1,36(sp)
      78:	02012903          	lw	s2,32(sp)
      7c:	03010113          	addi	sp,sp,48
      80:	00008067          	ret
      84:	00001537          	lui	a0,0x1
      88:	2d450513          	addi	a0,a0,724 # 12d4 <.Lanon.edf57770ec6efd366114906802a2c923.7>
      8c:	00a12423          	sw	a0,8(sp)
      90:	00100513          	li	a0,1
      94:	00a12623          	sw	a0,12(sp)
      98:	00012823          	sw	zero,16(sp)
      9c:	00001537          	lui	a0,0x1
      a0:	2a050513          	addi	a0,a0,672 # 12a0 <.Lanon.edf57770ec6efd366114906802a2c923.5>
      a4:	00a12c23          	sw	a0,24(sp)
      a8:	00012e23          	sw	zero,28(sp)
      ac:	00001537          	lui	a0,0x1
      b0:	2dc50593          	addi	a1,a0,732 # 12dc <.Lanon.edf57770ec6efd366114906802a2c923.8>
      b4:	0680006f          	j	11c <_ZN20simple_router_risc_v6virtio23VirtioMMIONetworkDevice19configure_virtqueue17h6e1b5c65dc542e08E.llvm.1659980649332766766+0x118>
      b8:	00001537          	lui	a0,0x1
      bc:	35050513          	addi	a0,a0,848 # 1350 <.Lanon.edf57770ec6efd366114906802a2c923.13>
      c0:	00a12423          	sw	a0,8(sp)
      c4:	00100513          	li	a0,1
      c8:	00a12623          	sw	a0,12(sp)
      cc:	00012823          	sw	zero,16(sp)
      d0:	00001537          	lui	a0,0x1
      d4:	2a050513          	addi	a0,a0,672 # 12a0 <.Lanon.edf57770ec6efd366114906802a2c923.5>
      d8:	00a12c23          	sw	a0,24(sp)
      dc:	00012e23          	sw	zero,28(sp)
      e0:	00001537          	lui	a0,0x1
      e4:	35850593          	addi	a1,a0,856 # 1358 <.Lanon.edf57770ec6efd366114906802a2c923.14>
      e8:	0340006f          	j	11c <_ZN20simple_router_risc_v6virtio23VirtioMMIONetworkDevice19configure_virtqueue17h6e1b5c65dc542e08E.llvm.1659980649332766766+0x118>
      ec:	00001537          	lui	a0,0x1
      f0:	30050513          	addi	a0,a0,768 # 1300 <.Lanon.edf57770ec6efd366114906802a2c923.10>
      f4:	00a12423          	sw	a0,8(sp)
      f8:	00100513          	li	a0,1
      fc:	00a12623          	sw	a0,12(sp)
     100:	00012823          	sw	zero,16(sp)
     104:	00001537          	lui	a0,0x1
     108:	2a050513          	addi	a0,a0,672 # 12a0 <.Lanon.edf57770ec6efd366114906802a2c923.5>
     10c:	00a12c23          	sw	a0,24(sp)
     110:	00012e23          	sw	zero,28(sp)
     114:	00001537          	lui	a0,0x1
     118:	30850593          	addi	a1,a0,776 # 1308 <.Lanon.edf57770ec6efd366114906802a2c923.11>
     11c:	00810513          	addi	a0,sp,8
     120:	00001097          	auipc	ra,0x1
     124:	d30080e7          	jalr	-720(ra) # e50 <_ZN4core9panicking9panic_fmt17hc433ad9e95c81f09E>
     128:	c0001073          	unimp

0000012c <_ZN20simple_router_risc_v9virtqueue16VirtQueueElement17as_network_packet17h310804dd2cd6bfe3E>:
     12c:	ff010113          	addi	sp,sp,-16
     130:	00112623          	sw	ra,12(sp)
     134:	0005a583          	lw	a1,0(a1)
     138:	00000097          	auipc	ra,0x0
     13c:	364080e7          	jalr	868(ra) # 49c <_ZN141_$LT$simple_router_risc_v..virtqueue..RawVirtQueueDescriptorPointer$u20$as$u20$simple_router_risc_v..virtqueue_network..NetworkDescriptor$GT$17as_network_packet17h04496b12929f9642E>
     140:	00c12083          	lw	ra,12(sp)
     144:	01010113          	addi	sp,sp,16
     148:	00008067          	ret

0000014c <_ZN20simple_router_risc_v9virtqueue16VirtQueueElement9copy_from17h2e88e9b12a4bcff4E>:
     14c:	00052703          	lw	a4,0(a0)
     150:	00472503          	lw	a0,4(a4)
     154:	00072503          	lw	a0,0(a4)
     158:	00872603          	lw	a2,8(a4)
     15c:	0005a583          	lw	a1,0(a1)
     160:	0045a683          	lw	a3,4(a1)
     164:	0005a683          	lw	a3,0(a1)
     168:	00872583          	lw	a1,8(a4)
     16c:	00b61863          	bne	a2,a1,17c <_ZN20simple_router_risc_v9virtqueue16VirtQueueElement9copy_from17h2e88e9b12a4bcff4E+0x30>
     170:	00068593          	mv	a1,a3
     174:	00001317          	auipc	t1,0x1
     178:	03430067          	jr	52(t1) # 11a8 <memcpy>
     17c:	00001537          	lui	a0,0x1
     180:	3ac50693          	addi	a3,a0,940 # 13ac <.Lanon.cb224257b9ac8f4dc414bb109df653f0.4>
     184:	00060513          	mv	a0,a2
     188:	00068613          	mv	a2,a3
     18c:	00001097          	auipc	ra,0x1
     190:	d2c080e7          	jalr	-724(ra) # eb8 <_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$15copy_from_slice17len_mismatch_fail17h4ae84e290ee18f5dE>
     194:	c0001073          	unimp

00000198 <_ZN20simple_router_risc_v9virtqueue15VirtQueueHandle3new17h4cfd822ae4a1d98aE>:
     198:	fd010113          	addi	sp,sp,-48
     19c:	02112623          	sw	ra,44(sp)
     1a0:	02812423          	sw	s0,40(sp)
     1a4:	02912223          	sw	s1,36(sp)
     1a8:	03212023          	sw	s2,32(sp)
     1ac:	01312e23          	sw	s3,28(sp)
     1b0:	01412c23          	sw	s4,24(sp)
     1b4:	01512a23          	sw	s5,20(sp)
     1b8:	01612823          	sw	s6,16(sp)
     1bc:	01712623          	sw	s7,12(sp)
     1c0:	01812423          	sw	s8,8(sp)
     1c4:	00060493          	mv	s1,a2
     1c8:	00058913          	mv	s2,a1
     1cc:	00050413          	mv	s0,a0
     1d0:	0e358513          	addi	a0,a1,227
     1d4:	01200593          	li	a1,18
     1d8:	02b50533          	mul	a0,a0,a1
     1dc:	00f50513          	addi	a0,a0,15
     1e0:	fffff5b7          	lui	a1,0xfffff
     1e4:	00b57533          	and	a0,a0,a1
     1e8:	00391713          	slli	a4,s2,0x3
     1ec:	00001b37          	lui	s6,0x1
     1f0:	005b0793          	addi	a5,s6,5 # 1005 <_ZN4core3str5count14do_count_chars17h629c24e475432246E+0xe5>
     1f4:	00062803          	lw	a6,0(a2)
     1f8:	00862603          	lw	a2,8(a2)
     1fc:	00f70733          	add	a4,a4,a5
     200:	00b77733          	and	a4,a4,a1
     204:	00e505b3          	add	a1,a0,a4
     208:	010609b3          	add	s3,a2,a6
     20c:	00c585b3          	add	a1,a1,a2
     210:	00b4a423          	sw	a1,8(s1)
     214:	00491593          	slli	a1,s2,0x4
     218:	00b985b3          	add	a1,s3,a1
     21c:	00258813          	addi	a6,a1,2 # fffff002 <__heap_size+0x8ffff002>
     220:	00458893          	addi	a7,a1,4
     224:	00a987b3          	add	a5,s3,a0
     228:	00278293          	addi	t0,a5,2
     22c:	00478313          	addi	t1,a5,4
     230:	01342023          	sw	s3,0(s0)
     234:	01242223          	sw	s2,4(s0)
     238:	02041823          	sh	zero,48(s0)
     23c:	01342423          	sw	s3,8(s0)
     240:	01242623          	sw	s2,12(s0)
     244:	00b42823          	sw	a1,16(s0)
     248:	01042a23          	sw	a6,20(s0)
     24c:	01142c23          	sw	a7,24(s0)
     250:	01242e23          	sw	s2,28(s0)
     254:	02f42023          	sw	a5,32(s0)
     258:	02542223          	sw	t0,36(s0)
     25c:	02642423          	sw	t1,40(s0)
     260:	02041623          	sh	zero,44(s0)
     264:	04090663          	beqz	s2,2b0 <_ZN20simple_router_risc_v9virtqueue15VirtQueueHandle3new17h4cfd822ae4a1d98aE+0x118>
     268:	00000593          	li	a1,0
     26c:	00169a13          	slli	s4,a3,0x1
     270:	00e78ab3          	add	s5,a5,a4
     274:	00a60533          	add	a0,a2,a0
     278:	00e50533          	add	a0,a0,a4
     27c:	01650bb3          	add	s7,a0,s6
     280:	0174a423          	sw	s7,8(s1)
     284:	00158c13          	addi	s8,a1,1
     288:	00098513          	mv	a0,s3
     28c:	000a8613          	mv	a2,s5
     290:	00000693          	li	a3,0
     294:	000a0713          	mv	a4,s4
     298:	00000097          	auipc	ra,0x0
     29c:	0f0080e7          	jalr	240(ra) # 388 <_ZN20simple_router_risc_v9virtqueue15VirtQueueHandle17update_descriptor17h70fdf81b6c4d0df7E>
     2a0:	016a8ab3          	add	s5,s5,s6
     2a4:	016b8bb3          	add	s7,s7,s6
     2a8:	000c0593          	mv	a1,s8
     2ac:	fd891ae3          	bne	s2,s8,280 <_ZN20simple_router_risc_v9virtqueue15VirtQueueHandle3new17h4cfd822ae4a1d98aE+0xe8>
     2b0:	00000593          	li	a1,0
     2b4:	40000493          	li	s1,1024
     2b8:	00158913          	addi	s2,a1,1
     2bc:	00040513          	mv	a0,s0
     2c0:	00000097          	auipc	ra,0x0
     2c4:	0b8080e7          	jalr	184(ra) # 378 <_ZN20simple_router_risc_v9virtqueue15VirtQueueHandle5offer17hce5684e7d6d4c836E>
     2c8:	00090593          	mv	a1,s2
     2cc:	fe9916e3          	bne	s2,s1,2b8 <_ZN20simple_router_risc_v9virtqueue15VirtQueueHandle3new17h4cfd822ae4a1d98aE+0x120>
     2d0:	8330000f          	fence.tso
     2d4:	02c12083          	lw	ra,44(sp)
     2d8:	02812403          	lw	s0,40(sp)
     2dc:	02412483          	lw	s1,36(sp)
     2e0:	02012903          	lw	s2,32(sp)
     2e4:	01c12983          	lw	s3,28(sp)
     2e8:	01812a03          	lw	s4,24(sp)
     2ec:	01412a83          	lw	s5,20(sp)
     2f0:	01012b03          	lw	s6,16(sp)
     2f4:	00c12b83          	lw	s7,12(sp)
     2f8:	00812c03          	lw	s8,8(sp)
     2fc:	03010113          	addi	sp,sp,48
     300:	00008067          	ret

00000304 <_ZN20simple_router_risc_v9virtqueue15VirtQueueHandle8try_take17h20214f8037b3f086E>:
     304:	ff010113          	addi	sp,sp,-16
     308:	00112623          	sw	ra,12(sp)
     30c:	00812423          	sw	s0,8(sp)
     310:	00912223          	sw	s1,4(sp)
     314:	00058493          	mv	s1,a1
     318:	00050413          	mv	s0,a0
     31c:	8330000f          	fence.tso
     320:	01c58513          	addi	a0,a1,28
     324:	00000097          	auipc	ra,0x0
     328:	0f8080e7          	jalr	248(ra) # 41c <_ZN20simple_router_risc_v9virtqueue14UsedRingHandle10try_remove17hba1abca5d1b7a213E>
     32c:	01051513          	slli	a0,a0,0x10
     330:	01055613          	srli	a2,a0,0x10
     334:	00100513          	li	a0,1
     338:	02a61263          	bne	a2,a0,35c <_ZN20simple_router_risc_v9virtqueue15VirtQueueHandle8try_take17h20214f8037b3f086E+0x58>
     33c:	0084a603          	lw	a2,8(s1)
     340:	01059693          	slli	a3,a1,0x10
     344:	0106d693          	srli	a3,a3,0x10
     348:	00469693          	slli	a3,a3,0x4
     34c:	00d60633          	add	a2,a2,a3
     350:	00c42223          	sw	a2,4(s0)
     354:	00b41423          	sh	a1,8(s0)
     358:	0080006f          	j	360 <_ZN20simple_router_risc_v9virtqueue15VirtQueueHandle8try_take17h20214f8037b3f086E+0x5c>
     35c:	00000513          	li	a0,0
     360:	00a42023          	sw	a0,0(s0)
     364:	00c12083          	lw	ra,12(sp)
     368:	00812403          	lw	s0,8(sp)
     36c:	00412483          	lw	s1,4(sp)
     370:	01010113          	addi	sp,sp,16
     374:	00008067          	ret

00000378 <_ZN20simple_router_risc_v9virtqueue15VirtQueueHandle5offer17hce5684e7d6d4c836E>:
     378:	00c50513          	addi	a0,a0,12
     37c:	8330000f          	fence.tso
     380:	00000317          	auipc	t1,0x0
     384:	03430067          	jr	52(t1) # 3b4 <_ZN20simple_router_risc_v9virtqueue19AvailableRingHandle7advance17h5abb4b82dcc3a728E>

00000388 <_ZN20simple_router_risc_v9virtqueue15VirtQueueHandle17update_descriptor17h70fdf81b6c4d0df7E>:
     388:	01059593          	slli	a1,a1,0x10
     38c:	0105d593          	srli	a1,a1,0x10
     390:	00459593          	slli	a1,a1,0x4
     394:	00a58533          	add	a0,a1,a0
     398:	00d52223          	sw	a3,4(a0)
     39c:	00c52023          	sw	a2,0(a0)
     3a0:	000015b7          	lui	a1,0x1
     3a4:	00b52423          	sw	a1,8(a0)
     3a8:	00e51623          	sh	a4,12(a0)
     3ac:	00051723          	sh	zero,14(a0)
     3b0:	00008067          	ret

000003b4 <_ZN20simple_router_risc_v9virtqueue19AvailableRingHandle7advance17h5abb4b82dcc3a728E>:
     3b4:	00852683          	lw	a3,8(a0)
     3b8:	00c52603          	lw	a2,12(a0)
     3bc:	0006d683          	lhu	a3,0(a3)
     3c0:	00052703          	lw	a4,0(a0)
     3c4:	01071713          	slli	a4,a4,0x10
     3c8:	01075713          	srli	a4,a4,0x10
     3cc:	02070863          	beqz	a4,3fc <_ZN20simple_router_risc_v9virtqueue19AvailableRingHandle7advance17h5abb4b82dcc3a728E+0x48>
     3d0:	02e6f6b3          	remu	a3,a3,a4
     3d4:	00169693          	slli	a3,a3,0x1
     3d8:	00d60633          	add	a2,a2,a3
     3dc:	00b61023          	sh	a1,0(a2)
     3e0:	8330000f          	fence.tso
     3e4:	00852503          	lw	a0,8(a0)
     3e8:	00051583          	lh	a1,0(a0)
     3ec:	00158593          	addi	a1,a1,1 # 1001 <_ZN4core3str5count14do_count_chars17h629c24e475432246E+0xe1>
     3f0:	00b51023          	sh	a1,0(a0)
     3f4:	8330000f          	fence.tso
     3f8:	00008067          	ret
     3fc:	00001537          	lui	a0,0x1
     400:	3d050513          	addi	a0,a0,976 # 13d0 <str.0>
     404:	000015b7          	lui	a1,0x1
     408:	3bc58613          	addi	a2,a1,956 # 13bc <.Lanon.cb224257b9ac8f4dc414bb109df653f0.7>
     40c:	03900593          	li	a1,57
     410:	00001097          	auipc	ra,0x1
     414:	9fc080e7          	jalr	-1540(ra) # e0c <_ZN4core9panicking5panic17hcced18483035532eE>
     418:	c0001073          	unimp

0000041c <_ZN20simple_router_risc_v9virtqueue14UsedRingHandle10try_remove17hba1abca5d1b7a213E>:
     41c:	00852583          	lw	a1,8(a0)
     420:	0005d603          	lhu	a2,0(a1)
     424:	01055583          	lhu	a1,16(a0)
     428:	00c59663          	bne	a1,a2,434 <_ZN20simple_router_risc_v9virtqueue14UsedRingHandle10try_remove17hba1abca5d1b7a213E+0x18>
     42c:	00000513          	li	a0,0
     430:	00008067          	ret
     434:	00052603          	lw	a2,0(a0)
     438:	01061693          	slli	a3,a2,0x10
     43c:	0106d693          	srli	a3,a3,0x10
     440:	02068e63          	beqz	a3,47c <_ZN20simple_router_risc_v9virtqueue14UsedRingHandle10try_remove17hba1abca5d1b7a213E+0x60>
     444:	000106b7          	lui	a3,0x10
     448:	fff68693          	addi	a3,a3,-1 # ffff <__stack_size+0xbfff>
     44c:	00c52703          	lw	a4,12(a0)
     450:	00d67633          	and	a2,a2,a3
     454:	02c5f7b3          	remu	a5,a1,a2
     458:	00379793          	slli	a5,a5,0x3
     45c:	00e78733          	add	a4,a5,a4
     460:	00072703          	lw	a4,0(a4)
     464:	00158593          	addi	a1,a1,1
     468:	00b51823          	sh	a1,16(a0)
     46c:	00d77533          	and	a0,a4,a3
     470:	02c575b3          	remu	a1,a0,a2
     474:	00100513          	li	a0,1
     478:	00008067          	ret
     47c:	00001537          	lui	a0,0x1
     480:	3d050513          	addi	a0,a0,976 # 13d0 <str.0>
     484:	000015b7          	lui	a1,0x1
     488:	40c58613          	addi	a2,a1,1036 # 140c <.Lanon.cb224257b9ac8f4dc414bb109df653f0.8>
     48c:	03900593          	li	a1,57
     490:	00001097          	auipc	ra,0x1
     494:	97c080e7          	jalr	-1668(ra) # e0c <_ZN4core9panicking5panic17hcced18483035532eE>
     498:	c0001073          	unimp

0000049c <_ZN141_$LT$simple_router_risc_v..virtqueue..RawVirtQueueDescriptorPointer$u20$as$u20$simple_router_risc_v..virtqueue_network..NetworkDescriptor$GT$17as_network_packet17h04496b12929f9642E>:
     49c:	0045a603          	lw	a2,4(a1)
     4a0:	0005a603          	lw	a2,0(a1)
     4a4:	0085a683          	lw	a3,8(a1)
     4a8:	00900713          	li	a4,9
     4ac:	02d77463          	bgeu	a4,a3,4d4 <_ZN141_$LT$simple_router_risc_v..virtqueue..RawVirtQueueDescriptorPointer$u20$as$u20$simple_router_risc_v..virtqueue_network..NetworkDescriptor$GT$17as_network_packet17h04496b12929f9642E+0x38>
     4b0:	0045a703          	lw	a4,4(a1)
     4b4:	0005a583          	lw	a1,0(a1)
     4b8:	00a60613          	addi	a2,a2,10
     4bc:	ff668693          	addi	a3,a3,-10
     4c0:	00b52023          	sw	a1,0(a0)
     4c4:	00e52223          	sw	a4,4(a0)
     4c8:	00c52423          	sw	a2,8(a0)
     4cc:	00d52623          	sw	a3,12(a0)
     4d0:	00008067          	ret
     4d4:	00001537          	lui	a0,0x1
     4d8:	36850513          	addi	a0,a0,872 # 1368 <.Lanon.cb224257b9ac8f4dc414bb109df653f0.0>
     4dc:	000015b7          	lui	a1,0x1
     4e0:	43458613          	addi	a2,a1,1076 # 1434 <.Lanon.cb224257b9ac8f4dc414bb109df653f0.10>
     4e4:	02300593          	li	a1,35
     4e8:	00001097          	auipc	ra,0x1
     4ec:	924080e7          	jalr	-1756(ra) # e0c <_ZN4core9panicking5panic17hcced18483035532eE>
     4f0:	c0001073          	unimp

000004f4 <_start>:
     4f4:	20004117          	auipc	sp,0x20004
     4f8:	b0c10113          	addi	sp,sp,-1268 # 20004000 <__stack_end>

000004fc <.Lpcrel_hi1>:
     4fc:	21000517          	auipc	a0,0x21000
     500:	b0450513          	addi	a0,a0,-1276 # 21000000 <__heap_start>
     504:	008000ef          	jal	ra,50c <main>

00000508 <rust_begin_unwind>:
     508:	0000006f          	j	508 <rust_begin_unwind>

0000050c <main>:
     50c:	e6010113          	addi	sp,sp,-416
     510:	18112e23          	sw	ra,412(sp)
     514:	18812c23          	sw	s0,408(sp)
     518:	18912a23          	sw	s1,404(sp)
     51c:	19212823          	sw	s2,400(sp)
     520:	19312623          	sw	s3,396(sp)
     524:	19412423          	sw	s4,392(sp)
     528:	19512223          	sw	s5,388(sp)
     52c:	19612023          	sw	s6,384(sp)
     530:	46000537          	lui	a0,0x46000
     534:	00a12023          	sw	a0,0(sp)
     538:	01000537          	lui	a0,0x1000
     53c:	00a12223          	sw	a0,4(sp)
     540:	00012423          	sw	zero,8(sp)
     544:	0a0044b7          	lui	s1,0xa004
     548:	e0048513          	addi	a0,s1,-512 # a003e00 <__stack_size+0x9fffe00>
     54c:	08a12023          	sw	a0,128(sp)
     550:	e004a503          	lw	a0,-512(s1)
     554:	e044a503          	lw	a0,-508(s1)
     558:	e604a823          	sw	zero,-400(s1)
     55c:	e704a503          	lw	a0,-400(s1)
     560:	00156513          	ori	a0,a0,1
     564:	e6a4a823          	sw	a0,-400(s1)
     568:	e704a503          	lw	a0,-400(s1)
     56c:	00256513          	ori	a0,a0,2
     570:	e6a4a823          	sw	a0,-400(s1)
     574:	e004aa23          	sw	zero,-492(s1)
     578:	e104a503          	lw	a0,-496(s1)
     57c:	e204a223          	sw	zero,-476(s1)
     580:	e204a023          	sw	zero,-480(s1)
     584:	00001537          	lui	a0,0x1
     588:	80050993          	addi	s3,a0,-2048 # 800 <_ZN4core3fmt9Formatter12pad_integral17h079d3faa4d086f53E+0x1c>
     58c:	e334a423          	sw	s3,-472(s1)
     590:	0f410413          	addi	s0,sp,244
     594:	0f810513          	addi	a0,sp,248
     598:	08010613          	addi	a2,sp,128
     59c:	00010693          	mv	a3,sp
     5a0:	00100713          	li	a4,1
     5a4:	00100913          	li	s2,1
     5a8:	00000593          	li	a1,0
     5ac:	00000097          	auipc	ra,0x0
     5b0:	a58080e7          	jalr	-1448(ra) # 4 <_ZN20simple_router_risc_v6virtio23VirtioMMIONetworkDevice19configure_virtqueue17h6e1b5c65dc542e08E.llvm.1659980649332766766>
     5b4:	12c10513          	addi	a0,sp,300
     5b8:	00100593          	li	a1,1
     5bc:	08010613          	addi	a2,sp,128
     5c0:	00010693          	mv	a3,sp
     5c4:	00000713          	li	a4,0
     5c8:	00000097          	auipc	ra,0x0
     5cc:	a3c080e7          	jalr	-1476(ra) # 4 <_ZN20simple_router_risc_v6virtio23VirtioMMIONetworkDevice19configure_virtqueue17h6e1b5c65dc542e08E.llvm.1659980649332766766>
     5d0:	08012503          	lw	a0,128(sp)
     5d4:	07052583          	lw	a1,112(a0)
     5d8:	0045e593          	ori	a1,a1,4
     5dc:	06b52823          	sw	a1,112(a0)
     5e0:	07052583          	lw	a1,112(a0)
     5e4:	04052823          	sw	zero,80(a0)
     5e8:	0ea12a23          	sw	a0,244(sp)
     5ec:	01010513          	addi	a0,sp,16
     5f0:	06c00613          	li	a2,108
     5f4:	00040593          	mv	a1,s0
     5f8:	00001097          	auipc	ra,0x1
     5fc:	bb0080e7          	jalr	-1104(ra) # 11a8 <memcpy>
     600:	c0048513          	addi	a0,s1,-1024
     604:	08a12023          	sw	a0,128(sp)
     608:	c004a503          	lw	a0,-1024(s1)
     60c:	c044a503          	lw	a0,-1020(s1)
     610:	c604a823          	sw	zero,-912(s1)
     614:	c704a503          	lw	a0,-912(s1)
     618:	00156513          	ori	a0,a0,1
     61c:	c6a4a823          	sw	a0,-912(s1)
     620:	c704a503          	lw	a0,-912(s1)
     624:	00256513          	ori	a0,a0,2
     628:	c6a4a823          	sw	a0,-912(s1)
     62c:	c004aa23          	sw	zero,-1004(s1)
     630:	c104a503          	lw	a0,-1008(s1)
     634:	c204a223          	sw	zero,-988(s1)
     638:	c204a023          	sw	zero,-992(s1)
     63c:	c334a423          	sw	s3,-984(s1)
     640:	0f410413          	addi	s0,sp,244
     644:	0f810513          	addi	a0,sp,248
     648:	08010613          	addi	a2,sp,128
     64c:	00010693          	mv	a3,sp
     650:	00100713          	li	a4,1
     654:	00000593          	li	a1,0
     658:	00000097          	auipc	ra,0x0
     65c:	9ac080e7          	jalr	-1620(ra) # 4 <_ZN20simple_router_risc_v6virtio23VirtioMMIONetworkDevice19configure_virtqueue17h6e1b5c65dc542e08E.llvm.1659980649332766766>
     660:	12c10513          	addi	a0,sp,300
     664:	00100593          	li	a1,1
     668:	08010613          	addi	a2,sp,128
     66c:	00010693          	mv	a3,sp
     670:	00000713          	li	a4,0
     674:	00000097          	auipc	ra,0x0
     678:	990080e7          	jalr	-1648(ra) # 4 <_ZN20simple_router_risc_v6virtio23VirtioMMIONetworkDevice19configure_virtqueue17h6e1b5c65dc542e08E.llvm.1659980649332766766>
     67c:	08012503          	lw	a0,128(sp)
     680:	07052583          	lw	a1,112(a0)
     684:	0045e593          	ori	a1,a1,4
     688:	06b52823          	sw	a1,112(a0)
     68c:	07052583          	lw	a1,112(a0)
     690:	04052823          	sw	zero,80(a0)
     694:	0ea12a23          	sw	a0,244(sp)
     698:	08010513          	addi	a0,sp,128
     69c:	06c00613          	li	a2,108
     6a0:	00040593          	mv	a1,s0
     6a4:	00001097          	auipc	ra,0x1
     6a8:	b04080e7          	jalr	-1276(ra) # 11a8 <memcpy>
     6ac:	08012503          	lw	a0,128(sp)
     6b0:	01410413          	addi	s0,sp,20
     6b4:	0b810493          	addi	s1,sp,184
     6b8:	05252823          	sw	s2,80(a0)
     6bc:	00d00993          	li	s3,13
     6c0:	00800a13          	li	s4,8
     6c4:	01d00a93          	li	s5,29
     6c8:	02100b13          	li	s6,33
     6cc:	01c0006f          	j	6e8 <main+0x1dc>
     6d0:	17411583          	lh	a1,372(sp)
     6d4:	00040513          	mv	a0,s0
     6d8:	00000097          	auipc	ra,0x0
     6dc:	ca0080e7          	jalr	-864(ra) # 378 <_ZN20simple_router_risc_v9virtqueue15VirtQueueHandle5offer17hce5684e7d6d4c836E>
     6e0:	01012503          	lw	a0,16(sp)
     6e4:	04052823          	sw	zero,80(a0)
     6e8:	16010513          	addi	a0,sp,352 # main loop
     6ec:	00040593          	mv	a1,s0
     6f0:	00000097          	auipc	ra,0x0
     6f4:	c14080e7          	jalr	-1004(ra) # 304 <_ZN20simple_router_risc_v9virtqueue15VirtQueueHandle8try_take17h20214f8037b3f086E>
     6f8:	16012503          	lw	a0,352(sp)
     6fc:	fe0506e3          	beqz	a0,6e8 <main+0x1dc>
     700:	16412503          	lw	a0,356(sp)
     704:	16811583          	lh	a1,360(sp)
     708:	16a12823          	sw	a0,368(sp)
     70c:	16b11a23          	sh	a1,372(sp)
     710:	0f010513          	addi	a0,sp,240
     714:	17010593          	addi	a1,sp,368
     718:	00000097          	auipc	ra,0x0
     71c:	a14080e7          	jalr	-1516(ra) # 12c <_ZN20simple_router_risc_v9virtqueue16VirtQueueElement17as_network_packet17h310804dd2cd6bfe3E>
     720:	0fc12583          	lw	a1,252(sp)
     724:	06b9fc63          	bgeu	s3,a1,79c <main+0x290>
     728:	0f812503          	lw	a0,248(sp)
     72c:	00d54603          	lbu	a2,13(a0)
     730:	00c54503          	lbu	a0,12(a0)
     734:	00861613          	slli	a2,a2,0x8
     738:	00a66533          	or	a0,a2,a0
     73c:	f9451ae3          	bne	a0,s4,6d0 <main+0x1c4>
     740:	06bafa63          	bgeu	s5,a1,7b4 <main+0x2a8>
     744:	08bb7463          	bgeu	s6,a1,7cc <main+0x2c0>
     748:	0f010513          	addi	a0,sp,240
     74c:	00048593          	mv	a1,s1
     750:	00000097          	auipc	ra,0x0
     754:	bb4080e7          	jalr	-1100(ra) # 304 <_ZN20simple_router_risc_v9virtqueue15VirtQueueHandle8try_take17h20214f8037b3f086E>
     758:	0f012503          	lw	a0,240(sp)
     75c:	f6050ae3          	beqz	a0,6d0 <main+0x1c4>
     760:	0f412503          	lw	a0,244(sp)
     764:	0f811583          	lh	a1,248(sp)
     768:	16a12c23          	sw	a0,376(sp)
     76c:	16b11e23          	sh	a1,380(sp)
     770:	17810513          	addi	a0,sp,376
     774:	17010593          	addi	a1,sp,368
     778:	00000097          	auipc	ra,0x0
     77c:	9d4080e7          	jalr	-1580(ra) # 14c <_ZN20simple_router_risc_v9virtqueue16VirtQueueElement9copy_from17h2e88e9b12a4bcff4E>
     780:	17c11583          	lh	a1,380(sp)
     784:	00048513          	mv	a0,s1
     788:	00000097          	auipc	ra,0x0
     78c:	bf0080e7          	jalr	-1040(ra) # 378 <_ZN20simple_router_risc_v9virtqueue15VirtQueueHandle5offer17hce5684e7d6d4c836E>
     790:	08012503          	lw	a0,128(sp)
     794:	05252823          	sw	s2,80(a0)
     798:	f39ff06f          	j	6d0 <main+0x1c4>
     79c:	00001537          	lui	a0,0x1
     7a0:	45050613          	addi	a2,a0,1104 # 1450 <.Lanon.3c3e003eece1c20446309970d2c0b5dd.5>
     7a4:	00e00513          	li	a0,14
     7a8:	00001097          	auipc	ra,0x1
     7ac:	98c080e7          	jalr	-1652(ra) # 1134 <_ZN4core5slice5index24slice_end_index_len_fail17hfe3b7ee5fe4a3114E>
     7b0:	c0001073          	unimp
     7b4:	00001537          	lui	a0,0x1
     7b8:	46050613          	addi	a2,a0,1120 # 1460 <.Lanon.3c3e003eece1c20446309970d2c0b5dd.7>
     7bc:	01e00513          	li	a0,30
     7c0:	00001097          	auipc	ra,0x1
     7c4:	974080e7          	jalr	-1676(ra) # 1134 <_ZN4core5slice5index24slice_end_index_len_fail17hfe3b7ee5fe4a3114E>
     7c8:	c0001073          	unimp
     7cc:	00001537          	lui	a0,0x1
     7d0:	47050613          	addi	a2,a0,1136 # 1470 <.Lanon.3c3e003eece1c20446309970d2c0b5dd.8>
     7d4:	02200513          	li	a0,34
     7d8:	00001097          	auipc	ra,0x1
     7dc:	95c080e7          	jalr	-1700(ra) # 1134 <_ZN4core5slice5index24slice_end_index_len_fail17hfe3b7ee5fe4a3114E>
     7e0:	c0001073          	unimp

000007e4 <_ZN4core3fmt9Formatter12pad_integral17h079d3faa4d086f53E>:
     7e4:	fd010113          	addi	sp,sp,-48
     7e8:	02112623          	sw	ra,44(sp)
     7ec:	02812423          	sw	s0,40(sp)
     7f0:	02912223          	sw	s1,36(sp)
     7f4:	03212023          	sw	s2,32(sp)
     7f8:	01312e23          	sw	s3,28(sp)
     7fc:	01412c23          	sw	s4,24(sp)
     800:	01512a23          	sw	s5,20(sp)
     804:	01612823          	sw	s6,16(sp)
     808:	01712623          	sw	s7,12(sp)
     80c:	01812423          	sw	s8,8(sp)
     810:	01912223          	sw	s9,4(sp)
     814:	01a12023          	sw	s10,0(sp)
     818:	00078493          	mv	s1,a5
     81c:	00070913          	mv	s2,a4
     820:	00068993          	mv	s3,a3
     824:	00060a93          	mv	s5,a2
     828:	00050413          	mv	s0,a0
     82c:	06058663          	beqz	a1,898 <_ZN4core3fmt9Formatter12pad_integral17h079d3faa4d086f53E+0xb4>
     830:	00042503          	lw	a0,0(s0)
     834:	00157593          	andi	a1,a0,1
     838:	00110a37          	lui	s4,0x110
     83c:	00058463          	beqz	a1,844 <_ZN4core3fmt9Formatter12pad_integral17h079d3faa4d086f53E+0x60>
     840:	02b00a13          	li	s4,43
     844:	00958bb3          	add	s7,a1,s1
     848:	00457513          	andi	a0,a0,4
     84c:	06050063          	beqz	a0,8ac <_ZN4core3fmt9Formatter12pad_integral17h079d3faa4d086f53E+0xc8>
     850:	01000513          	li	a0,16
     854:	06a9f463          	bgeu	s3,a0,8bc <_ZN4core3fmt9Formatter12pad_integral17h079d3faa4d086f53E+0xd8>
     858:	00000513          	li	a0,0
     85c:	02098463          	beqz	s3,884 <_ZN4core3fmt9Formatter12pad_integral17h079d3faa4d086f53E+0xa0>
     860:	fbf00593          	li	a1,-65
     864:	00098613          	mv	a2,s3
     868:	000a8693          	mv	a3,s5
     86c:	00068703          	lb	a4,0(a3)
     870:	00168693          	addi	a3,a3,1
     874:	00e5a733          	slt	a4,a1,a4
     878:	fff60613          	addi	a2,a2,-1
     87c:	00e50533          	add	a0,a0,a4
     880:	fe0616e3          	bnez	a2,86c <_ZN4core3fmt9Formatter12pad_integral17h079d3faa4d086f53E+0x88>
     884:	01750bb3          	add	s7,a0,s7
     888:	000a8b13          	mv	s6,s5
     88c:	00842503          	lw	a0,8(s0)
     890:	04051663          	bnez	a0,8dc <_ZN4core3fmt9Formatter12pad_integral17h079d3faa4d086f53E+0xf8>
     894:	08c0006f          	j	920 <_ZN4core3fmt9Formatter12pad_integral17h079d3faa4d086f53E+0x13c>
     898:	00042503          	lw	a0,0(s0)
     89c:	00148b93          	addi	s7,s1,1
     8a0:	02d00a13          	li	s4,45
     8a4:	00457513          	andi	a0,a0,4
     8a8:	fa0514e3          	bnez	a0,850 <_ZN4core3fmt9Formatter12pad_integral17h079d3faa4d086f53E+0x6c>
     8ac:	00000b13          	li	s6,0
     8b0:	00842503          	lw	a0,8(s0)
     8b4:	02051463          	bnez	a0,8dc <_ZN4core3fmt9Formatter12pad_integral17h079d3faa4d086f53E+0xf8>
     8b8:	0680006f          	j	920 <_ZN4core3fmt9Formatter12pad_integral17h079d3faa4d086f53E+0x13c>
     8bc:	000a8513          	mv	a0,s5
     8c0:	00098593          	mv	a1,s3
     8c4:	00000097          	auipc	ra,0x0
     8c8:	65c080e7          	jalr	1628(ra) # f20 <_ZN4core3str5count14do_count_chars17h629c24e475432246E>
     8cc:	01750bb3          	add	s7,a0,s7
     8d0:	000a8b13          	mv	s6,s5
     8d4:	00842503          	lw	a0,8(s0)
     8d8:	04050463          	beqz	a0,920 <_ZN4core3fmt9Formatter12pad_integral17h079d3faa4d086f53E+0x13c>
     8dc:	00c42d03          	lw	s10,12(s0)
     8e0:	05abf063          	bgeu	s7,s10,920 <_ZN4core3fmt9Formatter12pad_integral17h079d3faa4d086f53E+0x13c>
     8e4:	00044503          	lbu	a0,0(s0)
     8e8:	00857513          	andi	a0,a0,8
     8ec:	0a051063          	bnez	a0,98c <_ZN4core3fmt9Formatter12pad_integral17h079d3faa4d086f53E+0x1a8>
     8f0:	02044583          	lbu	a1,32(s0)
     8f4:	00300613          	li	a2,3
     8f8:	00100513          	li	a0,1
     8fc:	00c58463          	beq	a1,a2,904 <_ZN4core3fmt9Formatter12pad_integral17h079d3faa4d086f53E+0x120>
     900:	00058513          	mv	a0,a1
     904:	00357593          	andi	a1,a0,3
     908:	417d0533          	sub	a0,s10,s7
     90c:	0e058263          	beqz	a1,9f0 <_ZN4core3fmt9Formatter12pad_integral17h079d3faa4d086f53E+0x20c>
     910:	00100613          	li	a2,1
     914:	0ec59463          	bne	a1,a2,9fc <_ZN4core3fmt9Formatter12pad_integral17h079d3faa4d086f53E+0x218>
     918:	00000c13          	li	s8,0
     91c:	0ec0006f          	j	a08 <_ZN4core3fmt9Formatter12pad_integral17h079d3faa4d086f53E+0x224>
     920:	00040513          	mv	a0,s0
     924:	000a0593          	mv	a1,s4
     928:	000b0613          	mv	a2,s6
     92c:	00098693          	mv	a3,s3
     930:	00000097          	auipc	ra,0x0
     934:	274080e7          	jalr	628(ra) # ba4 <_ZN4core3fmt9Formatter12pad_integral12write_prefix17hf653853ccecb7f18E>
     938:	00100a93          	li	s5,1
     93c:	1a051c63          	bnez	a0,af4 <_ZN4core3fmt9Formatter12pad_integral17h079d3faa4d086f53E+0x310>
     940:	01c42583          	lw	a1,28(s0)
     944:	01842503          	lw	a0,24(s0)
     948:	00c5a303          	lw	t1,12(a1)
     94c:	00090593          	mv	a1,s2
     950:	00048613          	mv	a2,s1
     954:	02c12083          	lw	ra,44(sp)
     958:	02812403          	lw	s0,40(sp)
     95c:	02412483          	lw	s1,36(sp)
     960:	02012903          	lw	s2,32(sp)
     964:	01c12983          	lw	s3,28(sp)
     968:	01812a03          	lw	s4,24(sp)
     96c:	01412a83          	lw	s5,20(sp)
     970:	01012b03          	lw	s6,16(sp)
     974:	00c12b83          	lw	s7,12(sp)
     978:	00812c03          	lw	s8,8(sp)
     97c:	00412c83          	lw	s9,4(sp)
     980:	00012d03          	lw	s10,0(sp)
     984:	03010113          	addi	sp,sp,48
     988:	00030067          	jr	t1
     98c:	00442c03          	lw	s8,4(s0)
     990:	03000513          	li	a0,48
     994:	02044c83          	lbu	s9,32(s0)
     998:	00a42223          	sw	a0,4(s0)
     99c:	00100a93          	li	s5,1
     9a0:	03540023          	sb	s5,32(s0)
     9a4:	00040513          	mv	a0,s0
     9a8:	000a0593          	mv	a1,s4
     9ac:	000b0613          	mv	a2,s6
     9b0:	00098693          	mv	a3,s3
     9b4:	00000097          	auipc	ra,0x0
     9b8:	1f0080e7          	jalr	496(ra) # ba4 <_ZN4core3fmt9Formatter12pad_integral12write_prefix17hf653853ccecb7f18E>
     9bc:	12051c63          	bnez	a0,af4 <_ZN4core3fmt9Formatter12pad_integral17h079d3faa4d086f53E+0x310>
     9c0:	02044583          	lbu	a1,32(s0)
     9c4:	00300613          	li	a2,3
     9c8:	00100513          	li	a0,1
     9cc:	00c58463          	beq	a1,a2,9d4 <_ZN4core3fmt9Formatter12pad_integral17h079d3faa4d086f53E+0x1f0>
     9d0:	00058513          	mv	a0,a1
     9d4:	00357593          	andi	a1,a0,3
     9d8:	417d0533          	sub	a0,s10,s7
     9dc:	0c058863          	beqz	a1,aac <_ZN4core3fmt9Formatter12pad_integral17h079d3faa4d086f53E+0x2c8>
     9e0:	00100613          	li	a2,1
     9e4:	0cc59a63          	bne	a1,a2,ab8 <_ZN4core3fmt9Formatter12pad_integral17h079d3faa4d086f53E+0x2d4>
     9e8:	00000b13          	li	s6,0
     9ec:	0d80006f          	j	ac4 <_ZN4core3fmt9Formatter12pad_integral17h079d3faa4d086f53E+0x2e0>
     9f0:	00050c13          	mv	s8,a0
     9f4:	00000513          	li	a0,0
     9f8:	0100006f          	j	a08 <_ZN4core3fmt9Formatter12pad_integral17h079d3faa4d086f53E+0x224>
     9fc:	00150593          	addi	a1,a0,1
     a00:	00155513          	srli	a0,a0,0x1
     a04:	0015dc13          	srli	s8,a1,0x1
     a08:	01842a83          	lw	s5,24(s0)
     a0c:	01c42c83          	lw	s9,28(s0)
     a10:	00442b83          	lw	s7,4(s0)
     a14:	00150d13          	addi	s10,a0,1
     a18:	fffd0d13          	addi	s10,s10,-1
     a1c:	000d0e63          	beqz	s10,a38 <_ZN4core3fmt9Formatter12pad_integral17h079d3faa4d086f53E+0x254>
     a20:	010ca603          	lw	a2,16(s9)
     a24:	000a8513          	mv	a0,s5
     a28:	000b8593          	mv	a1,s7
     a2c:	000600e7          	jalr	a2
     a30:	fe0504e3          	beqz	a0,a18 <_ZN4core3fmt9Formatter12pad_integral17h079d3faa4d086f53E+0x234>
     a34:	0bc0006f          	j	af0 <_ZN4core3fmt9Formatter12pad_integral17h079d3faa4d086f53E+0x30c>
     a38:	00110537          	lui	a0,0x110
     a3c:	00100a93          	li	s5,1
     a40:	0aab8a63          	beq	s7,a0,af4 <_ZN4core3fmt9Formatter12pad_integral17h079d3faa4d086f53E+0x310>
     a44:	00040513          	mv	a0,s0
     a48:	000a0593          	mv	a1,s4
     a4c:	000b0613          	mv	a2,s6
     a50:	00098693          	mv	a3,s3
     a54:	00000097          	auipc	ra,0x0
     a58:	150080e7          	jalr	336(ra) # ba4 <_ZN4core3fmt9Formatter12pad_integral12write_prefix17hf653853ccecb7f18E>
     a5c:	08051c63          	bnez	a0,af4 <_ZN4core3fmt9Formatter12pad_integral17h079d3faa4d086f53E+0x310>
     a60:	01c42583          	lw	a1,28(s0)
     a64:	01842503          	lw	a0,24(s0)
     a68:	00c5a683          	lw	a3,12(a1)
     a6c:	00090593          	mv	a1,s2
     a70:	00048613          	mv	a2,s1
     a74:	000680e7          	jalr	a3
     a78:	06051e63          	bnez	a0,af4 <_ZN4core3fmt9Formatter12pad_integral17h079d3faa4d086f53E+0x310>
     a7c:	01842483          	lw	s1,24(s0)
     a80:	01c42403          	lw	s0,28(s0)
     a84:	00000913          	li	s2,0
     a88:	112c0863          	beq	s8,s2,b98 <_ZN4core3fmt9Formatter12pad_integral17h079d3faa4d086f53E+0x3b4>
     a8c:	01042603          	lw	a2,16(s0)
     a90:	00190913          	addi	s2,s2,1
     a94:	00048513          	mv	a0,s1
     a98:	000b8593          	mv	a1,s7
     a9c:	000600e7          	jalr	a2
     aa0:	fe0504e3          	beqz	a0,a88 <_ZN4core3fmt9Formatter12pad_integral17h079d3faa4d086f53E+0x2a4>
     aa4:	fff90513          	addi	a0,s2,-1
     aa8:	0f40006f          	j	b9c <_ZN4core3fmt9Formatter12pad_integral17h079d3faa4d086f53E+0x3b8>
     aac:	00050b13          	mv	s6,a0
     ab0:	00000513          	li	a0,0
     ab4:	0100006f          	j	ac4 <_ZN4core3fmt9Formatter12pad_integral17h079d3faa4d086f53E+0x2e0>
     ab8:	00150593          	addi	a1,a0,1 # 110001 <__stack_size+0x10c001>
     abc:	00155513          	srli	a0,a0,0x1
     ac0:	0015db13          	srli	s6,a1,0x1
     ac4:	01842a03          	lw	s4,24(s0)
     ac8:	01c42a83          	lw	s5,28(s0)
     acc:	00442983          	lw	s3,4(s0)
     ad0:	00150b93          	addi	s7,a0,1
     ad4:	fffb8b93          	addi	s7,s7,-1
     ad8:	040b8c63          	beqz	s7,b30 <_ZN4core3fmt9Formatter12pad_integral17h079d3faa4d086f53E+0x34c>
     adc:	010aa603          	lw	a2,16(s5)
     ae0:	000a0513          	mv	a0,s4
     ae4:	00098593          	mv	a1,s3
     ae8:	000600e7          	jalr	a2
     aec:	fe0504e3          	beqz	a0,ad4 <_ZN4core3fmt9Formatter12pad_integral17h079d3faa4d086f53E+0x2f0>
     af0:	00100a93          	li	s5,1
     af4:	000a8513          	mv	a0,s5
     af8:	02c12083          	lw	ra,44(sp)
     afc:	02812403          	lw	s0,40(sp)
     b00:	02412483          	lw	s1,36(sp)
     b04:	02012903          	lw	s2,32(sp)
     b08:	01c12983          	lw	s3,28(sp)
     b0c:	01812a03          	lw	s4,24(sp)
     b10:	01412a83          	lw	s5,20(sp)
     b14:	01012b03          	lw	s6,16(sp)
     b18:	00c12b83          	lw	s7,12(sp)
     b1c:	00812c03          	lw	s8,8(sp)
     b20:	00412c83          	lw	s9,4(sp)
     b24:	00012d03          	lw	s10,0(sp)
     b28:	03010113          	addi	sp,sp,48
     b2c:	00008067          	ret
     b30:	00110537          	lui	a0,0x110
     b34:	00100a93          	li	s5,1
     b38:	faa98ee3          	beq	s3,a0,af4 <_ZN4core3fmt9Formatter12pad_integral17h079d3faa4d086f53E+0x310>
     b3c:	01c42583          	lw	a1,28(s0)
     b40:	01842503          	lw	a0,24(s0)
     b44:	00c5a683          	lw	a3,12(a1)
     b48:	00090593          	mv	a1,s2
     b4c:	00048613          	mv	a2,s1
     b50:	000680e7          	jalr	a3
     b54:	fa0510e3          	bnez	a0,af4 <_ZN4core3fmt9Formatter12pad_integral17h079d3faa4d086f53E+0x310>
     b58:	01842483          	lw	s1,24(s0)
     b5c:	01c42903          	lw	s2,28(s0)
     b60:	00000a13          	li	s4,0
     b64:	034b0263          	beq	s6,s4,b88 <_ZN4core3fmt9Formatter12pad_integral17h079d3faa4d086f53E+0x3a4>
     b68:	01092603          	lw	a2,16(s2)
     b6c:	001a0a13          	addi	s4,s4,1 # 110001 <__stack_size+0x10c001>
     b70:	00048513          	mv	a0,s1
     b74:	00098593          	mv	a1,s3
     b78:	000600e7          	jalr	a2
     b7c:	fe0504e3          	beqz	a0,b64 <_ZN4core3fmt9Formatter12pad_integral17h079d3faa4d086f53E+0x380>
     b80:	fffa0513          	addi	a0,s4,-1
     b84:	f76568e3          	bltu	a0,s6,af4 <_ZN4core3fmt9Formatter12pad_integral17h079d3faa4d086f53E+0x310>
     b88:	00000a93          	li	s5,0
     b8c:	01842223          	sw	s8,4(s0)
     b90:	03940023          	sb	s9,32(s0)
     b94:	f61ff06f          	j	af4 <_ZN4core3fmt9Formatter12pad_integral17h079d3faa4d086f53E+0x310>
     b98:	000c0513          	mv	a0,s8
     b9c:	01853ab3          	sltu	s5,a0,s8
     ba0:	f55ff06f          	j	af4 <_ZN4core3fmt9Formatter12pad_integral17h079d3faa4d086f53E+0x310>

00000ba4 <_ZN4core3fmt9Formatter12pad_integral12write_prefix17hf653853ccecb7f18E>:
     ba4:	ff010113          	addi	sp,sp,-16
     ba8:	00112623          	sw	ra,12(sp)
     bac:	00812423          	sw	s0,8(sp)
     bb0:	00912223          	sw	s1,4(sp)
     bb4:	01212023          	sw	s2,0(sp)
     bb8:	00110737          	lui	a4,0x110
     bbc:	00068413          	mv	s0,a3
     bc0:	00060493          	mv	s1,a2
     bc4:	00050913          	mv	s2,a0
     bc8:	02e58063          	beq	a1,a4,be8 <_ZN4core3fmt9Formatter12pad_integral12write_prefix17hf653853ccecb7f18E+0x44>
     bcc:	01c92603          	lw	a2,28(s2)
     bd0:	01892503          	lw	a0,24(s2)
     bd4:	01062603          	lw	a2,16(a2)
     bd8:	000600e7          	jalr	a2
     bdc:	00050593          	mv	a1,a0
     be0:	00100513          	li	a0,1
     be4:	02059c63          	bnez	a1,c1c <_ZN4core3fmt9Formatter12pad_integral12write_prefix17hf653853ccecb7f18E+0x78>
     be8:	02048863          	beqz	s1,c18 <_ZN4core3fmt9Formatter12pad_integral12write_prefix17hf653853ccecb7f18E+0x74>
     bec:	01c92583          	lw	a1,28(s2)
     bf0:	01892503          	lw	a0,24(s2)
     bf4:	00c5a303          	lw	t1,12(a1)
     bf8:	00048593          	mv	a1,s1
     bfc:	00040613          	mv	a2,s0
     c00:	00c12083          	lw	ra,12(sp)
     c04:	00812403          	lw	s0,8(sp)
     c08:	00412483          	lw	s1,4(sp)
     c0c:	00012903          	lw	s2,0(sp)
     c10:	01010113          	addi	sp,sp,16
     c14:	00030067          	jr	t1
     c18:	00000513          	li	a0,0
     c1c:	00c12083          	lw	ra,12(sp)
     c20:	00812403          	lw	s0,8(sp)
     c24:	00412483          	lw	s1,4(sp)
     c28:	00012903          	lw	s2,0(sp)
     c2c:	01010113          	addi	sp,sp,16
     c30:	00008067          	ret

00000c34 <_ZN4core3fmt3num3imp7fmt_u3217hd851932318dbd463E.llvm.95414056202617162>:
     c34:	fc010113          	addi	sp,sp,-64
     c38:	02112e23          	sw	ra,60(sp)
     c3c:	02812c23          	sw	s0,56(sp)
     c40:	02912a23          	sw	s1,52(sp)
     c44:	00060693          	mv	a3,a2
     c48:	00455713          	srli	a4,a0,0x4
     c4c:	27100793          	li	a5,625
     c50:	02700613          	li	a2,39
     c54:	02f77663          	bgeu	a4,a5,c80 <_ZN4core3fmt3num3imp7fmt_u3217hd851932318dbd463E.llvm.95414056202617162+0x4c>
     c58:	06300713          	li	a4,99
     c5c:	0ca76863          	bltu	a4,a0,d2c <_ZN4core3fmt3num3imp7fmt_u3217hd851932318dbd463E.llvm.95414056202617162+0xf8>
     c60:	00a00713          	li	a4,10
     c64:	12e57463          	bgeu	a0,a4,d8c <_ZN4core3fmt3num3imp7fmt_u3217hd851932318dbd463E.llvm.95414056202617162+0x158>
     c68:	fff60613          	addi	a2,a2,-1
     c6c:	00d10713          	addi	a4,sp,13
     c70:	00c70733          	add	a4,a4,a2
     c74:	03050513          	addi	a0,a0,48 # 110030 <__stack_size+0x10c030>
     c78:	00a70023          	sb	a0,0(a4) # 110000 <__stack_size+0x10c000>
     c7c:	13c0006f          	j	db8 <_ZN4core3fmt3num3imp7fmt_u3217hd851932318dbd463E.llvm.95414056202617162+0x184>
     c80:	00000613          	li	a2,0
     c84:	d1b71737          	lui	a4,0xd1b71
     c88:	75970713          	addi	a4,a4,1881 # d1b71759 <__heap_size+0x61b71759>
     c8c:	000027b7          	lui	a5,0x2
     c90:	71078793          	addi	a5,a5,1808 # 2710 <anon.e13be147324aa5999b4bb022de90e1bd.15.llvm.5045827363982388448+0x10b4>
     c94:	00001837          	lui	a6,0x1
     c98:	47b80813          	addi	a6,a6,1147 # 147b <.Lanon.3c3e003eece1c20446309970d2c0b5dd.8+0xb>
     c9c:	06400893          	li	a7,100
     ca0:	000012b7          	lui	t0,0x1
     ca4:	48028293          	addi	t0,t0,1152 # 1480 <anon.8c79c99b70be038d6e5de7584a6c97d0.5.llvm.4773530063935947868>
     ca8:	00d10313          	addi	t1,sp,13
     cac:	05f5e3b7          	lui	t2,0x5f5e
     cb0:	0ff38393          	addi	t2,t2,255 # 5f5e0ff <__stack_size+0x5f5a0ff>
     cb4:	00050e13          	mv	t3,a0
     cb8:	02e53533          	mulhu	a0,a0,a4
     cbc:	00d55513          	srli	a0,a0,0xd
     cc0:	02f50eb3          	mul	t4,a0,a5
     cc4:	41de0eb3          	sub	t4,t3,t4
     cc8:	010e9f13          	slli	t5,t4,0x10
     ccc:	012f5f13          	srli	t5,t5,0x12
     cd0:	030f0f33          	mul	t5,t5,a6
     cd4:	011f5f13          	srli	t5,t5,0x11
     cd8:	001f1f93          	slli	t6,t5,0x1
     cdc:	031f0f33          	mul	t5,t5,a7
     ce0:	41ee8eb3          	sub	t4,t4,t5
     ce4:	011e9e93          	slli	t4,t4,0x11
     ce8:	010ede93          	srli	t4,t4,0x10
     cec:	005f8f33          	add	t5,t6,t0
     cf0:	00c30fb3          	add	t6,t1,a2
     cf4:	000f4403          	lbu	s0,0(t5)
     cf8:	001f0f03          	lb	t5,1(t5)
     cfc:	005e8eb3          	add	t4,t4,t0
     d00:	001e8483          	lb	s1,1(t4)
     d04:	000ece83          	lbu	t4,0(t4)
     d08:	03ef8223          	sb	t5,36(t6)
     d0c:	028f81a3          	sb	s0,35(t6)
     d10:	029f8323          	sb	s1,38(t6)
     d14:	03df82a3          	sb	t4,37(t6)
     d18:	ffc60613          	addi	a2,a2,-4
     d1c:	f9c3ece3          	bltu	t2,t3,cb4 <_ZN4core3fmt3num3imp7fmt_u3217hd851932318dbd463E.llvm.95414056202617162+0x80>
     d20:	02760613          	addi	a2,a2,39
     d24:	06300713          	li	a4,99
     d28:	f2a77ce3          	bgeu	a4,a0,c60 <_ZN4core3fmt3num3imp7fmt_u3217hd851932318dbd463E.llvm.95414056202617162+0x2c>
     d2c:	01051713          	slli	a4,a0,0x10
     d30:	01275713          	srli	a4,a4,0x12
     d34:	000017b7          	lui	a5,0x1
     d38:	47b78793          	addi	a5,a5,1147 # 147b <.Lanon.3c3e003eece1c20446309970d2c0b5dd.8+0xb>
     d3c:	02f70733          	mul	a4,a4,a5
     d40:	01175713          	srli	a4,a4,0x11
     d44:	06400793          	li	a5,100
     d48:	02f707b3          	mul	a5,a4,a5
     d4c:	40f50533          	sub	a0,a0,a5
     d50:	01151513          	slli	a0,a0,0x11
     d54:	01055513          	srli	a0,a0,0x10
     d58:	ffe60613          	addi	a2,a2,-2
     d5c:	000017b7          	lui	a5,0x1
     d60:	48078793          	addi	a5,a5,1152 # 1480 <anon.8c79c99b70be038d6e5de7584a6c97d0.5.llvm.4773530063935947868>
     d64:	00f50533          	add	a0,a0,a5
     d68:	00150783          	lb	a5,1(a0)
     d6c:	00054503          	lbu	a0,0(a0)
     d70:	00d10813          	addi	a6,sp,13
     d74:	00c80833          	add	a6,a6,a2
     d78:	00f800a3          	sb	a5,1(a6)
     d7c:	00a80023          	sb	a0,0(a6)
     d80:	00070513          	mv	a0,a4
     d84:	00a00713          	li	a4,10
     d88:	eee560e3          	bltu	a0,a4,c68 <_ZN4core3fmt3num3imp7fmt_u3217hd851932318dbd463E.llvm.95414056202617162+0x34>
     d8c:	00151513          	slli	a0,a0,0x1
     d90:	ffe60613          	addi	a2,a2,-2
     d94:	00001737          	lui	a4,0x1
     d98:	48070713          	addi	a4,a4,1152 # 1480 <anon.8c79c99b70be038d6e5de7584a6c97d0.5.llvm.4773530063935947868>
     d9c:	00e50533          	add	a0,a0,a4
     da0:	00150703          	lb	a4,1(a0)
     da4:	00054503          	lbu	a0,0(a0)
     da8:	00d10793          	addi	a5,sp,13
     dac:	00c787b3          	add	a5,a5,a2
     db0:	00e780a3          	sb	a4,1(a5)
     db4:	00a78023          	sb	a0,0(a5)
     db8:	00d10513          	addi	a0,sp,13
     dbc:	00c50733          	add	a4,a0,a2
     dc0:	02700513          	li	a0,39
     dc4:	40c507b3          	sub	a5,a0,a2
     dc8:	00001537          	lui	a0,0x1
     dcc:	54850613          	addi	a2,a0,1352 # 1548 <.Lanon.4c4bad0f8d019729c93543b0bd8bb498.17>
     dd0:	00068513          	mv	a0,a3
     dd4:	00000693          	li	a3,0
     dd8:	00000097          	auipc	ra,0x0
     ddc:	a0c080e7          	jalr	-1524(ra) # 7e4 <_ZN4core3fmt9Formatter12pad_integral17h079d3faa4d086f53E>
     de0:	03c12083          	lw	ra,60(sp)
     de4:	03812403          	lw	s0,56(sp)
     de8:	03412483          	lw	s1,52(sp)
     dec:	04010113          	addi	sp,sp,64
     df0:	00008067          	ret

00000df4 <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17hdcab8ae9e556eb52E>:
     df4:	00052503          	lw	a0,0(a0)
     df8:	00058613          	mv	a2,a1
     dfc:	00100593          	li	a1,1
     e00:	00000317          	auipc	t1,0x0
     e04:	e3430067          	jr	-460(t1) # c34 <_ZN4core3fmt3num3imp7fmt_u3217hd851932318dbd463E.llvm.95414056202617162>

00000e08 <_ZN4core3ptr28drop_in_place$LT$$RF$u64$GT$17hdd2e91bbfcd39356E.llvm.3771325329251126961>:
     e08:	00008067          	ret

00000e0c <_ZN4core9panicking5panic17hcced18483035532eE>:
     e0c:	fe010113          	addi	sp,sp,-32
     e10:	00a12c23          	sw	a0,24(sp)
     e14:	00b12e23          	sw	a1,28(sp)
     e18:	01810513          	addi	a0,sp,24
     e1c:	00a12023          	sw	a0,0(sp)
     e20:	00100513          	li	a0,1
     e24:	00a12223          	sw	a0,4(sp)
     e28:	00012423          	sw	zero,8(sp)
     e2c:	00001537          	lui	a0,0x1
     e30:	54850513          	addi	a0,a0,1352 # 1548 <.Lanon.4c4bad0f8d019729c93543b0bd8bb498.17>
     e34:	00a12823          	sw	a0,16(sp)
     e38:	00012a23          	sw	zero,20(sp)
     e3c:	00010513          	mv	a0,sp
     e40:	00060593          	mv	a1,a2
     e44:	00000097          	auipc	ra,0x0
     e48:	00c080e7          	jalr	12(ra) # e50 <_ZN4core9panicking9panic_fmt17hc433ad9e95c81f09E>
     e4c:	c0001073          	unimp

00000e50 <_ZN4core9panicking9panic_fmt17hc433ad9e95c81f09E>:
     e50:	fe010113          	addi	sp,sp,-32
     e54:	00001637          	lui	a2,0x1
     e58:	54860613          	addi	a2,a2,1352 # 1548 <.Lanon.4c4bad0f8d019729c93543b0bd8bb498.17>
     e5c:	00c12423          	sw	a2,8(sp)
     e60:	00001637          	lui	a2,0x1
     e64:	54860613          	addi	a2,a2,1352 # 1548 <.Lanon.4c4bad0f8d019729c93543b0bd8bb498.17>
     e68:	00c12623          	sw	a2,12(sp)
     e6c:	00a12823          	sw	a0,16(sp)
     e70:	00b12a23          	sw	a1,20(sp)
     e74:	00100513          	li	a0,1
     e78:	00a10c23          	sb	a0,24(sp)
     e7c:	00810513          	addi	a0,sp,8
     e80:	fffff097          	auipc	ra,0xfffff
     e84:	688080e7          	jalr	1672(ra) # 508 <rust_begin_unwind>
     e88:	c0001073          	unimp

00000e8c <_ZN4core3ops8function6FnOnce9call_once17h5bb81535b7ab4630E.llvm.4718113242963026730>:
     e8c:	00000097          	auipc	ra,0x0
     e90:	2b4080e7          	jalr	692(ra) # 1140 <_ZN4core5slice5index27slice_end_index_len_fail_rt17ha7a00b512b20a9d1E>
     e94:	c0001073          	unimp

00000e98 <_ZN4core10intrinsics17const_eval_select17hf3dcbb8d9e74b8d3E>:
     e98:	00000097          	auipc	ra,0x0
     e9c:	ff4080e7          	jalr	-12(ra) # e8c <_ZN4core3ops8function6FnOnce9call_once17h5bb81535b7ab4630E.llvm.4718113242963026730>
     ea0:	c0001073          	unimp

00000ea4 <_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h9c2f88862e7245f7E>:
     ea4:	1c5c1537          	lui	a0,0x1c5c1
     ea8:	49450513          	addi	a0,a0,1172 # 1c5c1494 <__stack_size+0x1c5bd494>
     eac:	274ec5b7          	lui	a1,0x274ec
     eb0:	96058593          	addi	a1,a1,-1696 # 274eb960 <__heap_start+0x64eb960>
     eb4:	00008067          	ret

00000eb8 <_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$15copy_from_slice17len_mismatch_fail17h4ae84e290ee18f5dE>:
     eb8:	fd010113          	addi	sp,sp,-48
     ebc:	00a12023          	sw	a0,0(sp)
     ec0:	00b12223          	sw	a1,4(sp)
     ec4:	00410513          	addi	a0,sp,4
     ec8:	02a12023          	sw	a0,32(sp)
     ecc:	00001537          	lui	a0,0x1
     ed0:	df450513          	addi	a0,a0,-524 # df4 <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17hdcab8ae9e556eb52E>
     ed4:	02a12223          	sw	a0,36(sp)
     ed8:	00010593          	mv	a1,sp
     edc:	02b12423          	sw	a1,40(sp)
     ee0:	02a12623          	sw	a0,44(sp)
     ee4:	00001537          	lui	a0,0x1
     ee8:	59c50513          	addi	a0,a0,1436 # 159c <.Lanon.86afcaa0d9c48b5a880df3f22fff44b2.23>
     eec:	00a12423          	sw	a0,8(sp)
     ef0:	00300513          	li	a0,3
     ef4:	00a12623          	sw	a0,12(sp)
     ef8:	00012823          	sw	zero,16(sp)
     efc:	02010513          	addi	a0,sp,32
     f00:	00a12c23          	sw	a0,24(sp)
     f04:	00200513          	li	a0,2
     f08:	00a12e23          	sw	a0,28(sp)
     f0c:	00810513          	addi	a0,sp,8
     f10:	00060593          	mv	a1,a2
     f14:	00000097          	auipc	ra,0x0
     f18:	f3c080e7          	jalr	-196(ra) # e50 <_ZN4core9panicking9panic_fmt17hc433ad9e95c81f09E>
     f1c:	c0001073          	unimp

00000f20 <_ZN4core3str5count14do_count_chars17h629c24e475432246E>:
     f20:	00050613          	mv	a2,a0
     f24:	00350513          	addi	a0,a0,3
     f28:	ffc57813          	andi	a6,a0,-4
     f2c:	40c80733          	sub	a4,a6,a2
     f30:	00e5ee63          	bltu	a1,a4,f4c <_ZN4core3str5count14do_count_chars17h629c24e475432246E+0x2c>
     f34:	40e58533          	sub	a0,a1,a4
     f38:	00453693          	sltiu	a3,a0,4
     f3c:	00400793          	li	a5,4
     f40:	00e7b7b3          	sltu	a5,a5,a4
     f44:	00d7e6b3          	or	a3,a5,a3
     f48:	02068663          	beqz	a3,f74 <_ZN4core3str5count14do_count_chars17h629c24e475432246E+0x54>
     f4c:	00000513          	li	a0,0
     f50:	02058063          	beqz	a1,f70 <_ZN4core3str5count14do_count_chars17h629c24e475432246E+0x50>
     f54:	fbf00693          	li	a3,-65
     f58:	00060703          	lb	a4,0(a2)
     f5c:	00160613          	addi	a2,a2,1
     f60:	00e6a733          	slt	a4,a3,a4
     f64:	fff58593          	addi	a1,a1,-1
     f68:	00e50533          	add	a0,a0,a4
     f6c:	fe0596e3          	bnez	a1,f58 <_ZN4core3str5count14do_count_chars17h629c24e475432246E+0x38>
     f70:	00008067          	ret
     f74:	00357693          	andi	a3,a0,3
     f78:	00000793          	li	a5,0
     f7c:	02070463          	beqz	a4,fa4 <_ZN4core3str5count14do_count_chars17h629c24e475432246E+0x84>
     f80:	410605b3          	sub	a1,a2,a6
     f84:	fbf00813          	li	a6,-65
     f88:	00060893          	mv	a7,a2
     f8c:	00088283          	lb	t0,0(a7)
     f90:	00188893          	addi	a7,a7,1
     f94:	005822b3          	slt	t0,a6,t0
     f98:	00158593          	addi	a1,a1,1
     f9c:	005787b3          	add	a5,a5,t0
     fa0:	fe0596e3          	bnez	a1,f8c <_ZN4core3str5count14do_count_chars17h629c24e475432246E+0x6c>
     fa4:	00e605b3          	add	a1,a2,a4
     fa8:	00000813          	li	a6,0
     fac:	02068463          	beqz	a3,fd4 <_ZN4core3str5count14do_count_chars17h629c24e475432246E+0xb4>
     fb0:	ffc57613          	andi	a2,a0,-4
     fb4:	00c58733          	add	a4,a1,a2
     fb8:	fbf00613          	li	a2,-65
     fbc:	00070883          	lb	a7,0(a4)
     fc0:	00170713          	addi	a4,a4,1
     fc4:	011628b3          	slt	a7,a2,a7
     fc8:	fff68693          	addi	a3,a3,-1
     fcc:	01180833          	add	a6,a6,a7
     fd0:	fe0696e3          	bnez	a3,fbc <_ZN4core3str5count14do_count_chars17h629c24e475432246E+0x9c>
     fd4:	00255293          	srli	t0,a0,0x2
     fd8:	01010537          	lui	a0,0x1010
     fdc:	10150713          	addi	a4,a0,257 # 1010101 <__stack_size+0x100c101>
     fe0:	00ff0537          	lui	a0,0xff0
     fe4:	0ff50613          	addi	a2,a0,255 # ff00ff <__stack_size+0xfec0ff>
     fe8:	00010537          	lui	a0,0x10
     fec:	00150693          	addi	a3,a0,1 # 10001 <__stack_size+0xc001>
     ff0:	00f80533          	add	a0,a6,a5
     ff4:	0340006f          	j	1028 <_ZN4core3str5count14do_count_chars17h629c24e475432246E+0x108>
     ff8:	00289293          	slli	t0,a7,0x2
     ffc:	005585b3          	add	a1,a1,t0
    1000:	411782b3          	sub	t0,a5,a7
    1004:	0038f893          	andi	a7,a7,3
    1008:	00c373b3          	and	t2,t1,a2
    100c:	00835313          	srli	t1,t1,0x8
    1010:	00c37333          	and	t1,t1,a2
    1014:	00730333          	add	t1,t1,t2
    1018:	02d30333          	mul	t1,t1,a3
    101c:	01035313          	srli	t1,t1,0x10
    1020:	00a30533          	add	a0,t1,a0
    1024:	0a089863          	bnez	a7,10d4 <_ZN4core3str5count14do_count_chars17h629c24e475432246E+0x1b4>
    1028:	f40284e3          	beqz	t0,f70 <_ZN4core3str5count14do_count_chars17h629c24e475432246E+0x50>
    102c:	00028793          	mv	a5,t0
    1030:	0c000813          	li	a6,192
    1034:	00028893          	mv	a7,t0
    1038:	0102e463          	bltu	t0,a6,1040 <_ZN4core3str5count14do_count_chars17h629c24e475432246E+0x120>
    103c:	0c000893          	li	a7,192
    1040:	0fc8f813          	andi	a6,a7,252
    1044:	00281293          	slli	t0,a6,0x2
    1048:	00558833          	add	a6,a1,t0
    104c:	00000313          	li	t1,0
    1050:	fa0284e3          	beqz	t0,ff8 <_ZN4core3str5count14do_count_chars17h629c24e475432246E+0xd8>
    1054:	00058293          	mv	t0,a1
    1058:	0002a383          	lw	t2,0(t0)
    105c:	fff3ce13          	not	t3,t2
    1060:	007e5e13          	srli	t3,t3,0x7
    1064:	0063d393          	srli	t2,t2,0x6
    1068:	0042ae83          	lw	t4,4(t0)
    106c:	007e63b3          	or	t2,t3,t2
    1070:	00e3f3b3          	and	t2,t2,a4
    1074:	00638333          	add	t1,t2,t1
    1078:	fffec393          	not	t2,t4
    107c:	0073d393          	srli	t2,t2,0x7
    1080:	006ede13          	srli	t3,t4,0x6
    1084:	0082ae83          	lw	t4,8(t0)
    1088:	01c3e3b3          	or	t2,t2,t3
    108c:	00e3f3b3          	and	t2,t2,a4
    1090:	00638333          	add	t1,t2,t1
    1094:	fffec393          	not	t2,t4
    1098:	0073d393          	srli	t2,t2,0x7
    109c:	006ede13          	srli	t3,t4,0x6
    10a0:	00c2ae83          	lw	t4,12(t0)
    10a4:	01c3e3b3          	or	t2,t2,t3
    10a8:	00e3f3b3          	and	t2,t2,a4
    10ac:	00638333          	add	t1,t2,t1
    10b0:	fffec393          	not	t2,t4
    10b4:	0073d393          	srli	t2,t2,0x7
    10b8:	006ede13          	srli	t3,t4,0x6
    10bc:	01c3e3b3          	or	t2,t2,t3
    10c0:	00e3f3b3          	and	t2,t2,a4
    10c4:	01028293          	addi	t0,t0,16
    10c8:	00638333          	add	t1,t2,t1
    10cc:	f90296e3          	bne	t0,a6,1058 <_ZN4core3str5count14do_count_chars17h629c24e475432246E+0x138>
    10d0:	f29ff06f          	j	ff8 <_ZN4core3str5count14do_count_chars17h629c24e475432246E+0xd8>
    10d4:	0c000593          	li	a1,192
    10d8:	00b7e463          	bltu	a5,a1,10e0 <_ZN4core3str5count14do_count_chars17h629c24e475432246E+0x1c0>
    10dc:	0c000793          	li	a5,192
    10e0:	00000593          	li	a1,0
    10e4:	0037f793          	andi	a5,a5,3
    10e8:	00279793          	slli	a5,a5,0x2
    10ec:	00082883          	lw	a7,0(a6)
    10f0:	00480813          	addi	a6,a6,4
    10f4:	fff8c293          	not	t0,a7
    10f8:	0072d293          	srli	t0,t0,0x7
    10fc:	0068d893          	srli	a7,a7,0x6
    1100:	0112e8b3          	or	a7,t0,a7
    1104:	00e8f8b3          	and	a7,a7,a4
    1108:	ffc78793          	addi	a5,a5,-4
    110c:	00b885b3          	add	a1,a7,a1
    1110:	fc079ee3          	bnez	a5,10ec <_ZN4core3str5count14do_count_chars17h629c24e475432246E+0x1cc>
    1114:	00c5f733          	and	a4,a1,a2
    1118:	0085d593          	srli	a1,a1,0x8
    111c:	00c5f5b3          	and	a1,a1,a2
    1120:	00e585b3          	add	a1,a1,a4
    1124:	02d585b3          	mul	a1,a1,a3
    1128:	0105d593          	srli	a1,a1,0x10
    112c:	00a58533          	add	a0,a1,a0
    1130:	00008067          	ret

00001134 <_ZN4core5slice5index24slice_end_index_len_fail17hfe3b7ee5fe4a3114E>:
    1134:	00000097          	auipc	ra,0x0
    1138:	d64080e7          	jalr	-668(ra) # e98 <_ZN4core10intrinsics17const_eval_select17hf3dcbb8d9e74b8d3E>
    113c:	c0001073          	unimp

00001140 <_ZN4core5slice5index27slice_end_index_len_fail_rt17ha7a00b512b20a9d1E>:
    1140:	fd010113          	addi	sp,sp,-48
    1144:	00a12023          	sw	a0,0(sp)
    1148:	00b12223          	sw	a1,4(sp)
    114c:	00010513          	mv	a0,sp
    1150:	02a12023          	sw	a0,32(sp)
    1154:	00001537          	lui	a0,0x1
    1158:	df450513          	addi	a0,a0,-524 # df4 <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17hdcab8ae9e556eb52E>
    115c:	02a12223          	sw	a0,36(sp)
    1160:	00410593          	addi	a1,sp,4
    1164:	02b12423          	sw	a1,40(sp)
    1168:	02a12623          	sw	a0,44(sp)
    116c:	00001537          	lui	a0,0x1
    1170:	64c50513          	addi	a0,a0,1612 # 164c <anon.e13be147324aa5999b4bb022de90e1bd.14.llvm.5045827363982388448>
    1174:	00a12423          	sw	a0,8(sp)
    1178:	00200513          	li	a0,2
    117c:	00a12623          	sw	a0,12(sp)
    1180:	00012823          	sw	zero,16(sp)
    1184:	02010593          	addi	a1,sp,32
    1188:	00b12c23          	sw	a1,24(sp)
    118c:	00a12e23          	sw	a0,28(sp)
    1190:	00001537          	lui	a0,0x1
    1194:	65c50593          	addi	a1,a0,1628 # 165c <anon.e13be147324aa5999b4bb022de90e1bd.15.llvm.5045827363982388448>
    1198:	00810513          	addi	a0,sp,8
    119c:	00000097          	auipc	ra,0x0
    11a0:	cb4080e7          	jalr	-844(ra) # e50 <_ZN4core9panicking9panic_fmt17hc433ad9e95c81f09E>
    11a4:	c0001073          	unimp

000011a8 <memcpy>:
    11a8:	00000317          	auipc	t1,0x0
    11ac:	00830067          	jr	8(t1) # 11b0 <_ZN17compiler_builtins3mem6memcpy17h87802b20b9605f3cE>

000011b0 <_ZN17compiler_builtins3mem6memcpy17h87802b20b9605f3cE>: # https://github.com/rust-lang/compiler-builtins/blob/b788cf35c07288c1818d6f9eae8cf0c608c1cfc6/src/mem/impls.rs#L100
    11b0:	00f00693          	li	a3,15          # WORD_COPY_THRESHOLD
    11b4:	08c6f863          	bgeu	a3,a2,1244 <_ZN17compiler_builtins3mem6memcpy17h87802b20b9605f3cE+0x94> # n >= WORD_COPY_THRESHOLD?
    11b8:	40a006b3          	neg	a3,a0          # a3 = dest.wrapping_neg()              sub a3, x0, a0
    11bc:	0036f693          	andi	a3,a3,3        # a3 = a3 & WORD_MASK
    11c0:	00d50733          	add	a4,a0,a3       # a4 (dest_end?) = a0 (dest) + a3 (diff to next word?)
    11c4:	02068063          	beqz	a3,11e4 <_ZN17compiler_builtins3mem6memcpy17h87802b20b9605f3cE+0x34> # fork?
    11c8:	00050793          	mv	a5,a0
    11cc:	00058813          	mv	a6,a1
    11d0:	00080883          	lb	a7,0(a6)
    11d4:	01178023          	sb	a7,0(a5)
    11d8:	00178793          	addi	a5,a5,1
    11dc:	00180813          	addi	a6,a6,1
    11e0:	fee7e8e3          	bltu	a5,a4,11d0 <_ZN17compiler_builtins3mem6memcpy17h87802b20b9605f3cE+0x20>
    11e4:	00d585b3          	add	a1,a1,a3
    11e8:	40d60633          	sub	a2,a2,a3
    11ec:	ffc67793          	andi	a5,a2,-4
    11f0:	0035f813          	andi	a6,a1,3
    11f4:	00f706b3          	add	a3,a4,a5
    11f8:	04080c63          	beqz	a6,1250 <_ZN17compiler_builtins3mem6memcpy17h87802b20b9605f3cE+0xa0>
    11fc:	ffc5f313          	andi	t1,a1,-4
    1200:	00032883          	lw	a7,0(t1)
    1204:	06f05463          	blez	a5,126c <_ZN17compiler_builtins3mem6memcpy17h87802b20b9605f3cE+0xbc>
    1208:	00359293          	slli	t0,a1,0x3
    120c:	0182f813          	andi	a6,t0,24
    1210:	405002b3          	neg	t0,t0
    1214:	0182f293          	andi	t0,t0,24
    1218:	00430313          	addi	t1,t1,4
    121c:	00032383          	lw	t2,0(t1)
    1220:	0108d8b3          	srl	a7,a7,a6
    1224:	00539e33          	sll	t3,t2,t0
    1228:	011e68b3          	or	a7,t3,a7
    122c:	01172023          	sw	a7,0(a4)
    1230:	00470713          	addi	a4,a4,4
    1234:	00430313          	addi	t1,t1,4
    1238:	00038893          	mv	a7,t2
    123c:	fed760e3          	bltu	a4,a3,121c <_ZN17compiler_builtins3mem6memcpy17h87802b20b9605f3cE+0x6c>
    1240:	02c0006f          	j	126c <_ZN17compiler_builtins3mem6memcpy17h87802b20b9605f3cE+0xbc>
    1244:	00050693          	mv	a3,a0
    1248:	02061863          	bnez	a2,1278 <_ZN17compiler_builtins3mem6memcpy17h87802b20b9605f3cE+0xc8>
    124c:	0440006f          	j	1290 <_ZN17compiler_builtins3mem6memcpy17h87802b20b9605f3cE+0xe0>
    1250:	00f05e63          	blez	a5,126c <_ZN17compiler_builtins3mem6memcpy17h87802b20b9605f3cE+0xbc>
    1254:	00058813          	mv	a6,a1
    1258:	00082883          	lw	a7,0(a6)       #  a7 = *src_usize
    125c:	01172023          	sw	a7,0(a4)       # *dest_usize = a7
    1260:	00470713          	addi	a4,a4,4        # dest_usize.add(1)
    1264:	00480813          	addi	a6,a6,4        # src_usize.add(1)
    1268:	fed768e3          	bltu	a4,a3,1258 <_ZN17compiler_builtins3mem6memcpy17h87802b20b9605f3cE+0xa8>    # if dest_usize < dest_end
    126c:	00f585b3          	add	a1,a1,a5
    1270:	00367613          	andi	a2,a2,3
    1274:	00060e63          	beqz	a2,1290 <_ZN17compiler_builtins3mem6memcpy17h87802b20b9605f3cE+0xe0>
    1278:	00c68633          	add	a2,a3,a2
    127c:	00058703          	lb	a4,0(a1)
    1280:	00e68023          	sb	a4,0(a3)
    1284:	00168693          	addi	a3,a3,1
    1288:	00158593          	addi	a1,a1,1
    128c:	fec6e8e3          	bltu	a3,a2,127c <_ZN17compiler_builtins3mem6memcpy17h87802b20b9605f3cE+0xcc>
    1290:	00008067          	ret
