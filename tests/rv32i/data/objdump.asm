
target/riscv32i-unknown-none-elf/release/simple_router_risc_v:     file format elf32-littleriscv


Disassembly of section .text:

00000004 <_start>:
       4:	20004117          	auipc	sp,0x20004
       8:	ffc10113          	addi	sp,sp,-4 # 20004000 <__stack_end>

0000000c <.Lpcrel_hi1>:
       c:	21000517          	auipc	a0,0x21000
      10:	ff450513          	addi	a0,a0,-12 # 21000000 <__heap_start>
      14:	008000ef          	jal	ra,1c <main>

00000018 <rust_begin_unwind>:
      18:	0000006f          	j	18 <rust_begin_unwind>

0000001c <main>:
      1c:	e6010113          	addi	sp,sp,-416
      20:	18112e23          	sw	ra,412(sp)
      24:	18812c23          	sw	s0,408(sp)
      28:	18912a23          	sw	s1,404(sp)
      2c:	19212823          	sw	s2,400(sp)
      30:	19312623          	sw	s3,396(sp)
      34:	19412423          	sw	s4,392(sp)
      38:	19512223          	sw	s5,388(sp)
      3c:	19612023          	sw	s6,384(sp)
      40:	46000537          	lui	a0,0x46000
      44:	00a12023          	sw	a0,0(sp)
      48:	01000537          	lui	a0,0x1000
      4c:	00a12223          	sw	a0,4(sp)
      50:	00012423          	sw	zero,8(sp)
      54:	0a0044b7          	lui	s1,0xa004
      58:	e0048513          	addi	a0,s1,-512 # a003e00 <__stack_size+0x9fffe00>
      5c:	08a12023          	sw	a0,128(sp)
      60:	e004a503          	lw	a0,-512(s1)
      64:	e044a503          	lw	a0,-508(s1)
      68:	e604a823          	sw	zero,-400(s1)
      6c:	e704a503          	lw	a0,-400(s1)
      70:	00156513          	ori	a0,a0,1
      74:	e6a4a823          	sw	a0,-400(s1)
      78:	e704a503          	lw	a0,-400(s1)
      7c:	00256513          	ori	a0,a0,2
      80:	e6a4a823          	sw	a0,-400(s1)
      84:	e004aa23          	sw	zero,-492(s1)
      88:	e104a503          	lw	a0,-496(s1)
      8c:	e204a223          	sw	zero,-476(s1)
      90:	e204a023          	sw	zero,-480(s1)
      94:	00001537          	lui	a0,0x1
      98:	80050993          	addi	s3,a0,-2048 # 800 <_ZN20simple_router_risc_v6virtio23VirtioMMIONetworkDevice19configure_virtqueue17h8622c35bd62bc04bE.llvm.16428960419682550808+0xb0>
      9c:	e334a423          	sw	s3,-472(s1)
      a0:	0f410413          	addi	s0,sp,244
      a4:	0f810513          	addi	a0,sp,248
      a8:	08010613          	addi	a2,sp,128
      ac:	00010693          	mv	a3,sp
      b0:	00100713          	li	a4,1
      b4:	00100913          	li	s2,1
      b8:	00000593          	li	a1,0
      bc:	00000097          	auipc	ra,0x0
      c0:	694080e7          	jalr	1684(ra) # 750 <_ZN20simple_router_risc_v6virtio23VirtioMMIONetworkDevice19configure_virtqueue17h8622c35bd62bc04bE.llvm.16428960419682550808>
      c4:	12c10513          	addi	a0,sp,300
      c8:	00100593          	li	a1,1
      cc:	08010613          	addi	a2,sp,128
      d0:	00010693          	mv	a3,sp
      d4:	00000713          	li	a4,0
      d8:	00000097          	auipc	ra,0x0
      dc:	678080e7          	jalr	1656(ra) # 750 <_ZN20simple_router_risc_v6virtio23VirtioMMIONetworkDevice19configure_virtqueue17h8622c35bd62bc04bE.llvm.16428960419682550808>
      e0:	08012503          	lw	a0,128(sp)
      e4:	07052583          	lw	a1,112(a0)
      e8:	0045e593          	ori	a1,a1,4
      ec:	06b52823          	sw	a1,112(a0)
      f0:	07052583          	lw	a1,112(a0)
      f4:	04052823          	sw	zero,80(a0)
      f8:	0ea12a23          	sw	a0,244(sp)
      fc:	01010513          	addi	a0,sp,16
     100:	06c00613          	li	a2,108
     104:	00040593          	mv	a1,s0
     108:	00001097          	auipc	ra,0x1
     10c:	200080e7          	jalr	512(ra) # 1308 <memcpy>
     110:	c0048513          	addi	a0,s1,-1024
     114:	08a12023          	sw	a0,128(sp)
     118:	c004a503          	lw	a0,-1024(s1)
     11c:	c044a503          	lw	a0,-1020(s1)
     120:	c604a823          	sw	zero,-912(s1)
     124:	c704a503          	lw	a0,-912(s1)
     128:	00156513          	ori	a0,a0,1
     12c:	c6a4a823          	sw	a0,-912(s1)
     130:	c704a503          	lw	a0,-912(s1)
     134:	00256513          	ori	a0,a0,2
     138:	c6a4a823          	sw	a0,-912(s1)
     13c:	c004aa23          	sw	zero,-1004(s1)
     140:	c104a503          	lw	a0,-1008(s1)
     144:	c204a223          	sw	zero,-988(s1)
     148:	c204a023          	sw	zero,-992(s1)
     14c:	c334a423          	sw	s3,-984(s1)
     150:	0f410413          	addi	s0,sp,244
     154:	0f810513          	addi	a0,sp,248
     158:	08010613          	addi	a2,sp,128
     15c:	00010693          	mv	a3,sp
     160:	00100713          	li	a4,1
     164:	00000593          	li	a1,0
     168:	00000097          	auipc	ra,0x0
     16c:	5e8080e7          	jalr	1512(ra) # 750 <_ZN20simple_router_risc_v6virtio23VirtioMMIONetworkDevice19configure_virtqueue17h8622c35bd62bc04bE.llvm.16428960419682550808>
     170:	12c10513          	addi	a0,sp,300
     174:	00100593          	li	a1,1
     178:	08010613          	addi	a2,sp,128
     17c:	00010693          	mv	a3,sp
     180:	00000713          	li	a4,0
     184:	00000097          	auipc	ra,0x0
     188:	5cc080e7          	jalr	1484(ra) # 750 <_ZN20simple_router_risc_v6virtio23VirtioMMIONetworkDevice19configure_virtqueue17h8622c35bd62bc04bE.llvm.16428960419682550808>
     18c:	08012503          	lw	a0,128(sp)
     190:	07052583          	lw	a1,112(a0)
     194:	0045e593          	ori	a1,a1,4
     198:	06b52823          	sw	a1,112(a0)
     19c:	07052583          	lw	a1,112(a0)
     1a0:	04052823          	sw	zero,80(a0)
     1a4:	0ea12a23          	sw	a0,244(sp)
     1a8:	08010513          	addi	a0,sp,128
     1ac:	06c00613          	li	a2,108
     1b0:	00040593          	mv	a1,s0
     1b4:	00001097          	auipc	ra,0x1
     1b8:	154080e7          	jalr	340(ra) # 1308 <memcpy>
     1bc:	08012503          	lw	a0,128(sp)
     1c0:	01410413          	addi	s0,sp,20
     1c4:	0b810493          	addi	s1,sp,184
     1c8:	05252823          	sw	s2,80(a0)
     1cc:	00d00993          	li	s3,13
     1d0:	00800a13          	li	s4,8
     1d4:	01d00a93          	li	s5,29
     1d8:	02100b13          	li	s6,33
     1dc:	01c0006f          	j	1f8 <main+0x1dc>
     1e0:	17411583          	lh	a1,372(sp)
     1e4:	00040513          	mv	a0,s0
     1e8:	00000097          	auipc	ra,0x0
     1ec:	360080e7          	jalr	864(ra) # 548 <_ZN20simple_router_risc_v9virtqueue15VirtQueueHandle5offer17h04d82f74d097d8dbE>
     1f0:	01012503          	lw	a0,16(sp)
     1f4:	04052823          	sw	zero,80(a0)
     1f8:	16010513          	addi	a0,sp,352
     1fc:	00040593          	mv	a1,s0
     200:	00000097          	auipc	ra,0x0
     204:	2d4080e7          	jalr	724(ra) # 4d4 <_ZN20simple_router_risc_v9virtqueue15VirtQueueHandle8try_take17h1c987c43e964c001E>
     208:	16012503          	lw	a0,352(sp)
     20c:	fe0506e3          	beqz	a0,1f8 <main+0x1dc>
     210:	16412503          	lw	a0,356(sp)
     214:	16811583          	lh	a1,360(sp)
     218:	16a12823          	sw	a0,368(sp)
     21c:	16b11a23          	sh	a1,372(sp)
     220:	0f010513          	addi	a0,sp,240
     224:	17010593          	addi	a1,sp,368
     228:	00000097          	auipc	ra,0x0
     22c:	0cc080e7          	jalr	204(ra) # 2f4 <_ZN20simple_router_risc_v9virtqueue16VirtQueueElement17as_network_packet17h0b91258277379ef5E>
     230:	0fc12583          	lw	a1,252(sp)
     234:	06b9fc63          	bgeu	s3,a1,2ac <main+0x290>
     238:	0f812503          	lw	a0,248(sp)
     23c:	00d54603          	lbu	a2,13(a0)
     240:	00c54503          	lbu	a0,12(a0)
     244:	00861613          	slli	a2,a2,0x8
     248:	00a66533          	or	a0,a2,a0
     24c:	f9451ae3          	bne	a0,s4,1e0 <main+0x1c4>
     250:	06bafa63          	bgeu	s5,a1,2c4 <main+0x2a8>
     254:	08bb7463          	bgeu	s6,a1,2dc <main+0x2c0>
     258:	0f010513          	addi	a0,sp,240
     25c:	00048593          	mv	a1,s1
     260:	00000097          	auipc	ra,0x0
     264:	274080e7          	jalr	628(ra) # 4d4 <_ZN20simple_router_risc_v9virtqueue15VirtQueueHandle8try_take17h1c987c43e964c001E>
     268:	0f012503          	lw	a0,240(sp)
     26c:	f6050ae3          	beqz	a0,1e0 <main+0x1c4>
     270:	0f412503          	lw	a0,244(sp)
     274:	0f811583          	lh	a1,248(sp)
     278:	16a12c23          	sw	a0,376(sp)
     27c:	16b11e23          	sh	a1,380(sp)
     280:	17810513          	addi	a0,sp,376
     284:	17010593          	addi	a1,sp,368
     288:	00000097          	auipc	ra,0x0
     28c:	08c080e7          	jalr	140(ra) # 314 <_ZN20simple_router_risc_v9virtqueue16VirtQueueElement9copy_from17h500e59a9fd739b20E>
     290:	17c11583          	lh	a1,380(sp)
     294:	00048513          	mv	a0,s1
     298:	00000097          	auipc	ra,0x0
     29c:	2b0080e7          	jalr	688(ra) # 548 <_ZN20simple_router_risc_v9virtqueue15VirtQueueHandle5offer17h04d82f74d097d8dbE>
     2a0:	08012503          	lw	a0,128(sp)
     2a4:	05252823          	sw	s2,80(a0)
     2a8:	f39ff06f          	j	1e0 <main+0x1c4>
     2ac:	00001537          	lui	a0,0x1
     2b0:	58c50613          	addi	a2,a0,1420 # 158c <.Lanon.3c3e003eece1c20446309970d2c0b5dd.5>
     2b4:	00e00513          	li	a0,14
     2b8:	00001097          	auipc	ra,0x1
     2bc:	fdc080e7          	jalr	-36(ra) # 1294 <_ZN4core5slice5index24slice_end_index_len_fail17hd8a83a9c6ff53337E>
     2c0:	c0001073          	unimp
     2c4:	00001537          	lui	a0,0x1
     2c8:	59c50613          	addi	a2,a0,1436 # 159c <.Lanon.3c3e003eece1c20446309970d2c0b5dd.7>
     2cc:	01e00513          	li	a0,30
     2d0:	00001097          	auipc	ra,0x1
     2d4:	fc4080e7          	jalr	-60(ra) # 1294 <_ZN4core5slice5index24slice_end_index_len_fail17hd8a83a9c6ff53337E>
     2d8:	c0001073          	unimp
     2dc:	00001537          	lui	a0,0x1
     2e0:	5ac50613          	addi	a2,a0,1452 # 15ac <.Lanon.3c3e003eece1c20446309970d2c0b5dd.8>
     2e4:	02200513          	li	a0,34
     2e8:	00001097          	auipc	ra,0x1
     2ec:	fac080e7          	jalr	-84(ra) # 1294 <_ZN4core5slice5index24slice_end_index_len_fail17hd8a83a9c6ff53337E>
     2f0:	c0001073          	unimp

000002f4 <_ZN20simple_router_risc_v9virtqueue16VirtQueueElement17as_network_packet17h0b91258277379ef5E>:
     2f4:	ff010113          	addi	sp,sp,-16
     2f8:	00112623          	sw	ra,12(sp)
     2fc:	0005a583          	lw	a1,0(a1)
     300:	00000097          	auipc	ra,0x0
     304:	3f8080e7          	jalr	1016(ra) # 6f8 <_ZN141_$LT$simple_router_risc_v..virtqueue..RawVirtQueueDescriptorPointer$u20$as$u20$simple_router_risc_v..virtqueue_network..NetworkDescriptor$GT$17as_network_packet17h5a53bbfa15a1a406E>
     308:	00c12083          	lw	ra,12(sp)
     30c:	01010113          	addi	sp,sp,16
     310:	00008067          	ret

00000314 <_ZN20simple_router_risc_v9virtqueue16VirtQueueElement9copy_from17h500e59a9fd739b20E>:
     314:	00052703          	lw	a4,0(a0)
     318:	00472503          	lw	a0,4(a4)
     31c:	00072503          	lw	a0,0(a4)
     320:	00872603          	lw	a2,8(a4)
     324:	0005a583          	lw	a1,0(a1)
     328:	0045a683          	lw	a3,4(a1)
     32c:	0005a683          	lw	a3,0(a1)
     330:	00872583          	lw	a1,8(a4)
     334:	00b61863          	bne	a2,a1,344 <_ZN20simple_router_risc_v9virtqueue16VirtQueueElement9copy_from17h500e59a9fd739b20E+0x30>
     338:	00068593          	mv	a1,a3
     33c:	00001317          	auipc	t1,0x1
     340:	fcc30067          	jr	-52(t1) # 1308 <memcpy>
     344:	00001537          	lui	a0,0x1
     348:	60050693          	addi	a3,a0,1536 # 1600 <.Lanon.755e23c5e3f7856c2d804c38cbbd2193.4>
     34c:	00060513          	mv	a0,a2
     350:	00068613          	mv	a2,a3
     354:	00001097          	auipc	ra,0x1
     358:	c5c080e7          	jalr	-932(ra) # fb0 <_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$15copy_from_slice17len_mismatch_fail17h005c86f544d3a78cE>
     35c:	c0001073          	unimp

00000360 <_ZN20simple_router_risc_v9virtqueue15VirtQueueHandle3new17h4ff811c4c3258a17E>:
     360:	fd010113          	addi	sp,sp,-48
     364:	02112623          	sw	ra,44(sp)
     368:	02812423          	sw	s0,40(sp)
     36c:	02912223          	sw	s1,36(sp)
     370:	03212023          	sw	s2,32(sp)
     374:	01312e23          	sw	s3,28(sp)
     378:	01412c23          	sw	s4,24(sp)
     37c:	01512a23          	sw	s5,20(sp)
     380:	01612823          	sw	s6,16(sp)
     384:	01712623          	sw	s7,12(sp)
     388:	01812423          	sw	s8,8(sp)
     38c:	00068993          	mv	s3,a3
     390:	00060493          	mv	s1,a2
     394:	00058913          	mv	s2,a1
     398:	00050413          	mv	s0,a0
     39c:	0e358513          	addi	a0,a1,227
     3a0:	01200593          	li	a1,18
     3a4:	00001097          	auipc	ra,0x1
     3a8:	f94080e7          	jalr	-108(ra) # 1338 <__mulsi3>
     3ac:	00f50513          	addi	a0,a0,15
     3b0:	fffff5b7          	lui	a1,0xfffff
     3b4:	00b57533          	and	a0,a0,a1
     3b8:	00391693          	slli	a3,s2,0x3
     3bc:	00001b37          	lui	s6,0x1
     3c0:	005b0713          	addi	a4,s6,5 # 1005 <_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$15copy_from_slice17len_mismatch_fail17h005c86f544d3a78cE+0x55>
     3c4:	0004a783          	lw	a5,0(s1)
     3c8:	0084a603          	lw	a2,8(s1)
     3cc:	00e686b3          	add	a3,a3,a4
     3d0:	00b6f6b3          	and	a3,a3,a1
     3d4:	00d505b3          	add	a1,a0,a3
     3d8:	00f60a33          	add	s4,a2,a5
     3dc:	00c585b3          	add	a1,a1,a2
     3e0:	00b4a423          	sw	a1,8(s1)
     3e4:	00491593          	slli	a1,s2,0x4
     3e8:	00ba05b3          	add	a1,s4,a1
     3ec:	00258793          	addi	a5,a1,2 # fffff002 <__heap_size+0x8ffff002>
     3f0:	00458813          	addi	a6,a1,4
     3f4:	00aa0733          	add	a4,s4,a0
     3f8:	00270893          	addi	a7,a4,2
     3fc:	00470293          	addi	t0,a4,4
     400:	01442023          	sw	s4,0(s0)
     404:	01242223          	sw	s2,4(s0)
     408:	02041823          	sh	zero,48(s0)
     40c:	01442423          	sw	s4,8(s0)
     410:	01242623          	sw	s2,12(s0)
     414:	00b42823          	sw	a1,16(s0)
     418:	00f42a23          	sw	a5,20(s0)
     41c:	01042c23          	sw	a6,24(s0)
     420:	01242e23          	sw	s2,28(s0)
     424:	02e42023          	sw	a4,32(s0)
     428:	03142223          	sw	a7,36(s0)
     42c:	02542423          	sw	t0,40(s0)
     430:	02041623          	sh	zero,44(s0)
     434:	04090663          	beqz	s2,480 <_ZN20simple_router_risc_v9virtqueue15VirtQueueHandle3new17h4ff811c4c3258a17E+0x120>
     438:	00000593          	li	a1,0
     43c:	00199993          	slli	s3,s3,0x1
     440:	00d70ab3          	add	s5,a4,a3
     444:	00a60533          	add	a0,a2,a0
     448:	00d50533          	add	a0,a0,a3
     44c:	01650bb3          	add	s7,a0,s6
     450:	0174a423          	sw	s7,8(s1)                      # update_descriptor loop
     454:	00158c13          	addi	s8,a1,1
     458:	000a0513          	mv	a0,s4
     45c:	000a8613          	mv	a2,s5
     460:	00000693          	li	a3,0
     464:	00098713          	mv	a4,s3
     468:	00000097          	auipc	ra,0x0
     46c:	0f0080e7          	jalr	240(ra) # 558 <_ZN20simple_router_risc_v9virtqueue15VirtQueueHandle17update_descriptor17h6a855061bcb33bccE>
     470:	016a8ab3          	add	s5,s5,s6
     474:	016b8bb3          	add	s7,s7,s6
     478:	000c0593          	mv	a1,s8
     47c:	fd891ae3          	bne	s2,s8,450 <_ZN20simple_router_risc_v9virtqueue15VirtQueueHandle3new17h4ff811c4c3258a17E+0xf0>
     480:	00000593          	li	a1,0
     484:	40000493          	li	s1,1024                       # offer loop
     488:	00158913          	addi	s2,a1,1
     48c:	00040513          	mv	a0,s0
     490:	00000097          	auipc	ra,0x0
     494:	0b8080e7          	jalr	184(ra) # 548 <_ZN20simple_router_risc_v9virtqueue15VirtQueueHandle5offer17h04d82f74d097d8dbE>
     498:	00090593          	mv	a1,s2
     49c:	fe9916e3          	bne	s2,s1,488 <_ZN20simple_router_risc_v9virtqueue15VirtQueueHandle3new17h4ff811c4c3258a17E+0x128>
     4a0:	8330000f          	fence.tso
     4a4:	02c12083          	lw	ra,44(sp)
     4a8:	02812403          	lw	s0,40(sp)
     4ac:	02412483          	lw	s1,36(sp)
     4b0:	02012903          	lw	s2,32(sp)
     4b4:	01c12983          	lw	s3,28(sp)
     4b8:	01812a03          	lw	s4,24(sp)
     4bc:	01412a83          	lw	s5,20(sp)
     4c0:	01012b03          	lw	s6,16(sp)
     4c4:	00c12b83          	lw	s7,12(sp)
     4c8:	00812c03          	lw	s8,8(sp)
     4cc:	03010113          	addi	sp,sp,48
     4d0:	00008067          	ret

000004d4 <_ZN20simple_router_risc_v9virtqueue15VirtQueueHandle8try_take17h1c987c43e964c001E>:
     4d4:	ff010113          	addi	sp,sp,-16
     4d8:	00112623          	sw	ra,12(sp)
     4dc:	00812423          	sw	s0,8(sp)
     4e0:	00912223          	sw	s1,4(sp)
     4e4:	00058493          	mv	s1,a1
     4e8:	00050413          	mv	s0,a0
     4ec:	8330000f          	fence.tso
     4f0:	01c58513          	addi	a0,a1,28
     4f4:	00000097          	auipc	ra,0x0
     4f8:	130080e7          	jalr	304(ra) # 624 <_ZN20simple_router_risc_v9virtqueue14UsedRingHandle10try_remove17ha740b505ce01006dE>
     4fc:	01051513          	slli	a0,a0,0x10
     500:	01055613          	srli	a2,a0,0x10
     504:	00100513          	li	a0,1
     508:	02a61263          	bne	a2,a0,52c <_ZN20simple_router_risc_v9virtqueue15VirtQueueHandle8try_take17h1c987c43e964c001E+0x58>
     50c:	0084a603          	lw	a2,8(s1)
     510:	01059693          	slli	a3,a1,0x10
     514:	0106d693          	srli	a3,a3,0x10
     518:	00469693          	slli	a3,a3,0x4
     51c:	00d60633          	add	a2,a2,a3
     520:	00c42223          	sw	a2,4(s0)
     524:	00b41423          	sh	a1,8(s0)
     528:	0080006f          	j	530 <_ZN20simple_router_risc_v9virtqueue15VirtQueueHandle8try_take17h1c987c43e964c001E+0x5c>
     52c:	00000513          	li	a0,0
     530:	00a42023          	sw	a0,0(s0)
     534:	00c12083          	lw	ra,12(sp)
     538:	00812403          	lw	s0,8(sp)
     53c:	00412483          	lw	s1,4(sp)
     540:	01010113          	addi	sp,sp,16
     544:	00008067          	ret

00000548 <_ZN20simple_router_risc_v9virtqueue15VirtQueueHandle5offer17h04d82f74d097d8dbE>:
     548:	00c50513          	addi	a0,a0,12
     54c:	8330000f          	fence.tso
     550:	00000317          	auipc	t1,0x0
     554:	03430067          	jr	52(t1) # 584 <_ZN20simple_router_risc_v9virtqueue19AvailableRingHandle7advance17he0cd57698d437cc7E>

00000558 <_ZN20simple_router_risc_v9virtqueue15VirtQueueHandle17update_descriptor17h6a855061bcb33bccE>:
     558:	01059593          	slli	a1,a1,0x10
     55c:	0105d593          	srli	a1,a1,0x10
     560:	00459593          	slli	a1,a1,0x4
     564:	00a58533          	add	a0,a1,a0
     568:	00d52223          	sw	a3,4(a0)
     56c:	00c52023          	sw	a2,0(a0)
     570:	000015b7          	lui	a1,0x1
     574:	00b52423          	sw	a1,8(a0)
     578:	00e51623          	sh	a4,12(a0)
     57c:	00051723          	sh	zero,14(a0)
     580:	00008067          	ret

00000584 <_ZN20simple_router_risc_v9virtqueue19AvailableRingHandle7advance17he0cd57698d437cc7E>:
     584:	ff010113          	addi	sp,sp,-16
     588:	00112623          	sw	ra,12(sp)
     58c:	00812423          	sw	s0,8(sp)
     590:	00912223          	sw	s1,4(sp)
     594:	01212023          	sw	s2,0(sp)
     598:	00050413          	mv	s0,a0
     59c:	00852503          	lw	a0,8(a0)
     5a0:	00c42903          	lw	s2,12(s0)
     5a4:	00055503          	lhu	a0,0(a0)
     5a8:	00042603          	lw	a2,0(s0)
     5ac:	01061613          	slli	a2,a2,0x10
     5b0:	01065613          	srli	a2,a2,0x10
     5b4:	04060863          	beqz	a2,604 <_ZN20simple_router_risc_v9virtqueue19AvailableRingHandle7advance17he0cd57698d437cc7E+0x80>
     5b8:	00058493          	mv	s1,a1
     5bc:	00060593          	mv	a1,a2
     5c0:	00001097          	auipc	ra,0x1
     5c4:	d58080e7          	jalr	-680(ra) # 1318 <__umodsi3>
     5c8:	00151513          	slli	a0,a0,0x1
     5cc:	00a90533          	add	a0,s2,a0
     5d0:	00951023          	sh	s1,0(a0)
     5d4:	8330000f          	fence.tso
     5d8:	00842503          	lw	a0,8(s0)
     5dc:	00051583          	lh	a1,0(a0)
     5e0:	00158593          	addi	a1,a1,1 # 1001 <_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$15copy_from_slice17len_mismatch_fail17h005c86f544d3a78cE+0x51>
     5e4:	00b51023          	sh	a1,0(a0)
     5e8:	8330000f          	fence.tso
     5ec:	00c12083          	lw	ra,12(sp)
     5f0:	00812403          	lw	s0,8(sp)
     5f4:	00412483          	lw	s1,4(sp)
     5f8:	00012903          	lw	s2,0(sp)
     5fc:	01010113          	addi	sp,sp,16
     600:	00008067          	ret
     604:	00001537          	lui	a0,0x1
     608:	62050513          	addi	a0,a0,1568 # 1620 <str.0>
     60c:	000015b7          	lui	a1,0x1
     610:	61058613          	addi	a2,a1,1552 # 1610 <.Lanon.755e23c5e3f7856c2d804c38cbbd2193.7>
     614:	03900593          	li	a1,57
     618:	00001097          	auipc	ra,0x1
     61c:	8ec080e7          	jalr	-1812(ra) # f04 <_ZN4core9panicking5panic17hcabcc163a9651165E>
     620:	c0001073          	unimp

00000624 <_ZN20simple_router_risc_v9virtqueue14UsedRingHandle10try_remove17ha740b505ce01006dE>:
     624:	fe010113          	addi	sp,sp,-32
     628:	00112e23          	sw	ra,28(sp)
     62c:	00812c23          	sw	s0,24(sp)
     630:	00912a23          	sw	s1,20(sp)
     634:	01212823          	sw	s2,16(sp)
     638:	01312623          	sw	s3,12(sp)
     63c:	01412423          	sw	s4,8(sp)
     640:	00050413          	mv	s0,a0
     644:	00852503          	lw	a0,8(a0)
     648:	00055503          	lhu	a0,0(a0)
     64c:	01045483          	lhu	s1,16(s0)
     650:	00a49663          	bne	s1,a0,65c <_ZN20simple_router_risc_v9virtqueue14UsedRingHandle10try_remove17ha740b505ce01006dE+0x38>
     654:	00000513          	li	a0,0
     658:	0600006f          	j	6b8 <_ZN20simple_router_risc_v9virtqueue14UsedRingHandle10try_remove17ha740b505ce01006dE+0x94>
     65c:	00042503          	lw	a0,0(s0)
     660:	01051593          	slli	a1,a0,0x10
     664:	0105d593          	srli	a1,a1,0x10
     668:	06058863          	beqz	a1,6d8 <_ZN20simple_router_risc_v9virtqueue14UsedRingHandle10try_remove17ha740b505ce01006dE+0xb4>
     66c:	00c42983          	lw	s3,12(s0)
     670:	000105b7          	lui	a1,0x10
     674:	fff58a13          	addi	s4,a1,-1 # ffff <__stack_size+0xbfff>
     678:	01457933          	and	s2,a0,s4
     67c:	00048513          	mv	a0,s1
     680:	00090593          	mv	a1,s2
     684:	00001097          	auipc	ra,0x1
     688:	c94080e7          	jalr	-876(ra) # 1318 <__umodsi3>
     68c:	00351513          	slli	a0,a0,0x3
     690:	01350533          	add	a0,a0,s3
     694:	00052503          	lw	a0,0(a0)
     698:	00148593          	addi	a1,s1,1
     69c:	00b41823          	sh	a1,16(s0)
     6a0:	01457533          	and	a0,a0,s4
     6a4:	00090593          	mv	a1,s2
     6a8:	00001097          	auipc	ra,0x1
     6ac:	c70080e7          	jalr	-912(ra) # 1318 <__umodsi3>
     6b0:	00050593          	mv	a1,a0
     6b4:	00100513          	li	a0,1
     6b8:	01c12083          	lw	ra,28(sp)
     6bc:	01812403          	lw	s0,24(sp)
     6c0:	01412483          	lw	s1,20(sp)
     6c4:	01012903          	lw	s2,16(sp)
     6c8:	00c12983          	lw	s3,12(sp)
     6cc:	00812a03          	lw	s4,8(sp)
     6d0:	02010113          	addi	sp,sp,32
     6d4:	00008067          	ret
     6d8:	00001537          	lui	a0,0x1
     6dc:	62050513          	addi	a0,a0,1568 # 1620 <str.0>
     6e0:	000015b7          	lui	a1,0x1
     6e4:	65c58613          	addi	a2,a1,1628 # 165c <.Lanon.755e23c5e3f7856c2d804c38cbbd2193.8>
     6e8:	03900593          	li	a1,57
     6ec:	00001097          	auipc	ra,0x1
     6f0:	818080e7          	jalr	-2024(ra) # f04 <_ZN4core9panicking5panic17hcabcc163a9651165E>
     6f4:	c0001073          	unimp

000006f8 <_ZN141_$LT$simple_router_risc_v..virtqueue..RawVirtQueueDescriptorPointer$u20$as$u20$simple_router_risc_v..virtqueue_network..NetworkDescriptor$GT$17as_network_packet17h5a53bbfa15a1a406E>:
     6f8:	0045a603          	lw	a2,4(a1)
     6fc:	0005a603          	lw	a2,0(a1)
     700:	0085a683          	lw	a3,8(a1)
     704:	00900713          	li	a4,9
     708:	02d77463          	bgeu	a4,a3,730 <_ZN141_$LT$simple_router_risc_v..virtqueue..RawVirtQueueDescriptorPointer$u20$as$u20$simple_router_risc_v..virtqueue_network..NetworkDescriptor$GT$17as_network_packet17h5a53bbfa15a1a406E+0x38>
     70c:	0045a703          	lw	a4,4(a1)
     710:	0005a583          	lw	a1,0(a1)
     714:	00a60613          	addi	a2,a2,10
     718:	ff668693          	addi	a3,a3,-10
     71c:	00b52023          	sw	a1,0(a0)
     720:	00e52223          	sw	a4,4(a0)
     724:	00c52423          	sw	a2,8(a0)
     728:	00d52623          	sw	a3,12(a0)
     72c:	00008067          	ret
     730:	00001537          	lui	a0,0x1
     734:	5bc50513          	addi	a0,a0,1468 # 15bc <.Lanon.755e23c5e3f7856c2d804c38cbbd2193.0>
     738:	000015b7          	lui	a1,0x1
     73c:	68458613          	addi	a2,a1,1668 # 1684 <.Lanon.755e23c5e3f7856c2d804c38cbbd2193.10>
     740:	02300593          	li	a1,35
     744:	00000097          	auipc	ra,0x0
     748:	7c0080e7          	jalr	1984(ra) # f04 <_ZN4core9panicking5panic17hcabcc163a9651165E>
     74c:	c0001073          	unimp

00000750 <_ZN20simple_router_risc_v6virtio23VirtioMMIONetworkDevice19configure_virtqueue17h8622c35bd62bc04bE.llvm.16428960419682550808>:
     750:	fd010113          	addi	sp,sp,-48
     754:	02112623          	sw	ra,44(sp)
     758:	02812423          	sw	s0,40(sp)
     75c:	02912223          	sw	s1,36(sp)
     760:	03212023          	sw	s2,32(sp)
     764:	00062483          	lw	s1,0(a2)
     768:	02b4a823          	sw	a1,48(s1)
     76c:	0404a583          	lw	a1,64(s1)      # queue_pfn (should be 0)
     770:	06059063          	bnez	a1,7d0 <_ZN20simple_router_risc_v6virtio23VirtioMMIONetworkDevice19configure_virtqueue17h8622c35bd62bc04bE.llvm.16428960419682550808+0x80>
     774:	00050413          	mv	s0,a0
     778:	0344a503          	lw	a0,52(s1)      # queue_num_max (should be >= 1024)
     77c:	08050463          	beqz	a0,804 <_ZN20simple_router_risc_v6virtio23VirtioMMIONetworkDevice19configure_virtqueue17h8622c35bd62bc04bE.llvm.16428960419682550808+0xb4>
     780:	40000913          	li	s2,1024
     784:	0b256a63          	bltu	a0,s2,838 <_ZN20simple_router_risc_v6virtio23VirtioMMIONetworkDevice19configure_virtqueue17h8622c35bd62bc04bE.llvm.16428960419682550808+0xe8>
     788:	40000593          	li	a1,1024
     78c:	00040513          	mv	a0,s0
     790:	00068613          	mv	a2,a3
     794:	00070693          	mv	a3,a4
     798:	00000097          	auipc	ra,0x0
     79c:	bc8080e7          	jalr	-1080(ra) # 360 <_ZN20simple_router_risc_v9virtqueue15VirtQueueHandle3new17h4ff811c4c3258a17E>
     7a0:	0324ac23          	sw	s2,56(s1)
     7a4:	00001537          	lui	a0,0x1
     7a8:	02a4ae23          	sw	a0,60(s1)
     7ac:	00042503          	lw	a0,0(s0)
     7b0:	00b55513          	srli	a0,a0,0xb
     7b4:	04a4a023          	sw	a0,64(s1)
     7b8:	02c12083          	lw	ra,44(sp)
     7bc:	02812403          	lw	s0,40(sp)
     7c0:	02412483          	lw	s1,36(sp)
     7c4:	02012903          	lw	s2,32(sp)
     7c8:	03010113          	addi	sp,sp,48
     7cc:	00008067          	ret
     7d0:	00001537          	lui	a0,0x1
     7d4:	6c850513          	addi	a0,a0,1736 # 16c8 <.Lanon.eb86771b89d17ca25f8bd475580baf3a.7>
     7d8:	00a12423          	sw	a0,8(sp)
     7dc:	00100513          	li	a0,1
     7e0:	00a12623          	sw	a0,12(sp)
     7e4:	00012823          	sw	zero,16(sp)
     7e8:	00001537          	lui	a0,0x1
     7ec:	69450513          	addi	a0,a0,1684 # 1694 <.Lanon.eb86771b89d17ca25f8bd475580baf3a.5>
     7f0:	00a12c23          	sw	a0,24(sp)
     7f4:	00012e23          	sw	zero,28(sp)
     7f8:	00001537          	lui	a0,0x1
     7fc:	6d050593          	addi	a1,a0,1744 # 16d0 <.Lanon.eb86771b89d17ca25f8bd475580baf3a.8>
     800:	0680006f          	j	868 <_ZN20simple_router_risc_v6virtio23VirtioMMIONetworkDevice19configure_virtqueue17h8622c35bd62bc04bE.llvm.16428960419682550808+0x118>
     804:	00001537          	lui	a0,0x1
     808:	74450513          	addi	a0,a0,1860 # 1744 <.Lanon.eb86771b89d17ca25f8bd475580baf3a.13>
     80c:	00a12423          	sw	a0,8(sp)
     810:	00100513          	li	a0,1
     814:	00a12623          	sw	a0,12(sp)
     818:	00012823          	sw	zero,16(sp)
     81c:	00001537          	lui	a0,0x1
     820:	69450513          	addi	a0,a0,1684 # 1694 <.Lanon.eb86771b89d17ca25f8bd475580baf3a.5>
     824:	00a12c23          	sw	a0,24(sp)
     828:	00012e23          	sw	zero,28(sp)
     82c:	00001537          	lui	a0,0x1
     830:	74c50593          	addi	a1,a0,1868 # 174c <.Lanon.eb86771b89d17ca25f8bd475580baf3a.14>
     834:	0340006f          	j	868 <_ZN20simple_router_risc_v6virtio23VirtioMMIONetworkDevice19configure_virtqueue17h8622c35bd62bc04bE.llvm.16428960419682550808+0x118>
     838:	00001537          	lui	a0,0x1
     83c:	6f450513          	addi	a0,a0,1780 # 16f4 <.Lanon.eb86771b89d17ca25f8bd475580baf3a.10>
     840:	00a12423          	sw	a0,8(sp)
     844:	00100513          	li	a0,1
     848:	00a12623          	sw	a0,12(sp)
     84c:	00012823          	sw	zero,16(sp)
     850:	00001537          	lui	a0,0x1
     854:	69450513          	addi	a0,a0,1684 # 1694 <.Lanon.eb86771b89d17ca25f8bd475580baf3a.5>
     858:	00a12c23          	sw	a0,24(sp)
     85c:	00012e23          	sw	zero,28(sp)
     860:	00001537          	lui	a0,0x1
     864:	6fc50593          	addi	a1,a0,1788 # 16fc <.Lanon.eb86771b89d17ca25f8bd475580baf3a.11>
     868:	00810513          	addi	a0,sp,8
     86c:	00000097          	auipc	ra,0x0
     870:	6dc080e7          	jalr	1756(ra) # f48 <_ZN4core9panicking9panic_fmt17h0a1f7e9b00d01db1E>
     874:	c0001073          	unimp

00000878 <_ZN4core3fmt9Formatter12pad_integral17h53dd5693591e204cE>:
     878:	fd010113          	addi	sp,sp,-48
     87c:	02112623          	sw	ra,44(sp)
     880:	02812423          	sw	s0,40(sp)
     884:	02912223          	sw	s1,36(sp)
     888:	03212023          	sw	s2,32(sp)
     88c:	01312e23          	sw	s3,28(sp)
     890:	01412c23          	sw	s4,24(sp)
     894:	01512a23          	sw	s5,20(sp)
     898:	01612823          	sw	s6,16(sp)
     89c:	01712623          	sw	s7,12(sp)
     8a0:	01812423          	sw	s8,8(sp)
     8a4:	01912223          	sw	s9,4(sp)
     8a8:	01a12023          	sw	s10,0(sp)
     8ac:	00078493          	mv	s1,a5
     8b0:	00070913          	mv	s2,a4
     8b4:	00068993          	mv	s3,a3
     8b8:	00060a93          	mv	s5,a2
     8bc:	00050413          	mv	s0,a0
     8c0:	06058663          	beqz	a1,92c <_ZN4core3fmt9Formatter12pad_integral17h53dd5693591e204cE+0xb4>
     8c4:	00042503          	lw	a0,0(s0)
     8c8:	00157593          	andi	a1,a0,1
     8cc:	00110a37          	lui	s4,0x110
     8d0:	00058463          	beqz	a1,8d8 <_ZN4core3fmt9Formatter12pad_integral17h53dd5693591e204cE+0x60>
     8d4:	02b00a13          	li	s4,43
     8d8:	00958bb3          	add	s7,a1,s1
     8dc:	00457513          	andi	a0,a0,4
     8e0:	06050063          	beqz	a0,940 <_ZN4core3fmt9Formatter12pad_integral17h53dd5693591e204cE+0xc8>
     8e4:	01000513          	li	a0,16
     8e8:	06a9f463          	bgeu	s3,a0,950 <_ZN4core3fmt9Formatter12pad_integral17h53dd5693591e204cE+0xd8>
     8ec:	00000513          	li	a0,0
     8f0:	02098463          	beqz	s3,918 <_ZN4core3fmt9Formatter12pad_integral17h53dd5693591e204cE+0xa0>
     8f4:	fbf00593          	li	a1,-65
     8f8:	00098613          	mv	a2,s3
     8fc:	000a8693          	mv	a3,s5
     900:	00068703          	lb	a4,0(a3)
     904:	00168693          	addi	a3,a3,1
     908:	00e5a733          	slt	a4,a1,a4
     90c:	fff60613          	addi	a2,a2,-1
     910:	00e50533          	add	a0,a0,a4
     914:	fe0616e3          	bnez	a2,900 <_ZN4core3fmt9Formatter12pad_integral17h53dd5693591e204cE+0x88>
     918:	01750bb3          	add	s7,a0,s7
     91c:	000a8b13          	mv	s6,s5
     920:	00842503          	lw	a0,8(s0)
     924:	04051663          	bnez	a0,970 <_ZN4core3fmt9Formatter12pad_integral17h53dd5693591e204cE+0xf8>
     928:	08c0006f          	j	9b4 <_ZN4core3fmt9Formatter12pad_integral17h53dd5693591e204cE+0x13c>
     92c:	00042503          	lw	a0,0(s0)
     930:	00148b93          	addi	s7,s1,1
     934:	02d00a13          	li	s4,45
     938:	00457513          	andi	a0,a0,4
     93c:	fa0514e3          	bnez	a0,8e4 <_ZN4core3fmt9Formatter12pad_integral17h53dd5693591e204cE+0x6c>
     940:	00000b13          	li	s6,0
     944:	00842503          	lw	a0,8(s0)
     948:	02051463          	bnez	a0,970 <_ZN4core3fmt9Formatter12pad_integral17h53dd5693591e204cE+0xf8>
     94c:	0680006f          	j	9b4 <_ZN4core3fmt9Formatter12pad_integral17h53dd5693591e204cE+0x13c>
     950:	000a8513          	mv	a0,s5
     954:	00098593          	mv	a1,s3
     958:	00000097          	auipc	ra,0x0
     95c:	6c0080e7          	jalr	1728(ra) # 1018 <_ZN4core3str5count14do_count_chars17h467d32cb0cb52437E>
     960:	01750bb3          	add	s7,a0,s7
     964:	000a8b13          	mv	s6,s5
     968:	00842503          	lw	a0,8(s0)
     96c:	04050463          	beqz	a0,9b4 <_ZN4core3fmt9Formatter12pad_integral17h53dd5693591e204cE+0x13c>
     970:	00c42d03          	lw	s10,12(s0)
     974:	05abf063          	bgeu	s7,s10,9b4 <_ZN4core3fmt9Formatter12pad_integral17h53dd5693591e204cE+0x13c>
     978:	00044503          	lbu	a0,0(s0)
     97c:	00857513          	andi	a0,a0,8
     980:	0a051063          	bnez	a0,a20 <_ZN4core3fmt9Formatter12pad_integral17h53dd5693591e204cE+0x1a8>
     984:	02044583          	lbu	a1,32(s0)
     988:	00300613          	li	a2,3
     98c:	00100513          	li	a0,1
     990:	00c58463          	beq	a1,a2,998 <_ZN4core3fmt9Formatter12pad_integral17h53dd5693591e204cE+0x120>
     994:	00058513          	mv	a0,a1
     998:	00357593          	andi	a1,a0,3
     99c:	417d0533          	sub	a0,s10,s7
     9a0:	0e058263          	beqz	a1,a84 <_ZN4core3fmt9Formatter12pad_integral17h53dd5693591e204cE+0x20c>
     9a4:	00100613          	li	a2,1
     9a8:	0ec59463          	bne	a1,a2,a90 <_ZN4core3fmt9Formatter12pad_integral17h53dd5693591e204cE+0x218>
     9ac:	00000c13          	li	s8,0
     9b0:	0ec0006f          	j	a9c <_ZN4core3fmt9Formatter12pad_integral17h53dd5693591e204cE+0x224>
     9b4:	00040513          	mv	a0,s0
     9b8:	000a0593          	mv	a1,s4
     9bc:	000b0613          	mv	a2,s6
     9c0:	00098693          	mv	a3,s3
     9c4:	00000097          	auipc	ra,0x0
     9c8:	274080e7          	jalr	628(ra) # c38 <_ZN4core3fmt9Formatter12pad_integral12write_prefix17ha16bc6d3c7950701E>
     9cc:	00100a93          	li	s5,1
     9d0:	1a051c63          	bnez	a0,b88 <_ZN4core3fmt9Formatter12pad_integral17h53dd5693591e204cE+0x310>
     9d4:	01c42583          	lw	a1,28(s0)
     9d8:	01842503          	lw	a0,24(s0)
     9dc:	00c5a303          	lw	t1,12(a1)
     9e0:	00090593          	mv	a1,s2
     9e4:	00048613          	mv	a2,s1
     9e8:	02c12083          	lw	ra,44(sp)
     9ec:	02812403          	lw	s0,40(sp)
     9f0:	02412483          	lw	s1,36(sp)
     9f4:	02012903          	lw	s2,32(sp)
     9f8:	01c12983          	lw	s3,28(sp)
     9fc:	01812a03          	lw	s4,24(sp)
     a00:	01412a83          	lw	s5,20(sp)
     a04:	01012b03          	lw	s6,16(sp)
     a08:	00c12b83          	lw	s7,12(sp)
     a0c:	00812c03          	lw	s8,8(sp)
     a10:	00412c83          	lw	s9,4(sp)
     a14:	00012d03          	lw	s10,0(sp)
     a18:	03010113          	addi	sp,sp,48
     a1c:	00030067          	jr	t1
     a20:	00442c03          	lw	s8,4(s0)
     a24:	03000513          	li	a0,48
     a28:	02044c83          	lbu	s9,32(s0)
     a2c:	00a42223          	sw	a0,4(s0)
     a30:	00100a93          	li	s5,1
     a34:	03540023          	sb	s5,32(s0)
     a38:	00040513          	mv	a0,s0
     a3c:	000a0593          	mv	a1,s4
     a40:	000b0613          	mv	a2,s6
     a44:	00098693          	mv	a3,s3
     a48:	00000097          	auipc	ra,0x0
     a4c:	1f0080e7          	jalr	496(ra) # c38 <_ZN4core3fmt9Formatter12pad_integral12write_prefix17ha16bc6d3c7950701E>
     a50:	12051c63          	bnez	a0,b88 <_ZN4core3fmt9Formatter12pad_integral17h53dd5693591e204cE+0x310>
     a54:	02044583          	lbu	a1,32(s0)
     a58:	00300613          	li	a2,3
     a5c:	00100513          	li	a0,1
     a60:	00c58463          	beq	a1,a2,a68 <_ZN4core3fmt9Formatter12pad_integral17h53dd5693591e204cE+0x1f0>
     a64:	00058513          	mv	a0,a1
     a68:	00357593          	andi	a1,a0,3
     a6c:	417d0533          	sub	a0,s10,s7
     a70:	0c058863          	beqz	a1,b40 <_ZN4core3fmt9Formatter12pad_integral17h53dd5693591e204cE+0x2c8>
     a74:	00100613          	li	a2,1
     a78:	0cc59a63          	bne	a1,a2,b4c <_ZN4core3fmt9Formatter12pad_integral17h53dd5693591e204cE+0x2d4>
     a7c:	00000b13          	li	s6,0
     a80:	0d80006f          	j	b58 <_ZN4core3fmt9Formatter12pad_integral17h53dd5693591e204cE+0x2e0>
     a84:	00050c13          	mv	s8,a0
     a88:	00000513          	li	a0,0
     a8c:	0100006f          	j	a9c <_ZN4core3fmt9Formatter12pad_integral17h53dd5693591e204cE+0x224>
     a90:	00150593          	addi	a1,a0,1
     a94:	00155513          	srli	a0,a0,0x1
     a98:	0015dc13          	srli	s8,a1,0x1
     a9c:	01842a83          	lw	s5,24(s0)
     aa0:	01c42c83          	lw	s9,28(s0)
     aa4:	00442b83          	lw	s7,4(s0)
     aa8:	00150d13          	addi	s10,a0,1
     aac:	fffd0d13          	addi	s10,s10,-1
     ab0:	000d0e63          	beqz	s10,acc <_ZN4core3fmt9Formatter12pad_integral17h53dd5693591e204cE+0x254>
     ab4:	010ca603          	lw	a2,16(s9)
     ab8:	000a8513          	mv	a0,s5
     abc:	000b8593          	mv	a1,s7
     ac0:	000600e7          	jalr	a2
     ac4:	fe0504e3          	beqz	a0,aac <_ZN4core3fmt9Formatter12pad_integral17h53dd5693591e204cE+0x234>
     ac8:	0bc0006f          	j	b84 <_ZN4core3fmt9Formatter12pad_integral17h53dd5693591e204cE+0x30c>
     acc:	00110537          	lui	a0,0x110
     ad0:	00100a93          	li	s5,1
     ad4:	0aab8a63          	beq	s7,a0,b88 <_ZN4core3fmt9Formatter12pad_integral17h53dd5693591e204cE+0x310>
     ad8:	00040513          	mv	a0,s0
     adc:	000a0593          	mv	a1,s4
     ae0:	000b0613          	mv	a2,s6
     ae4:	00098693          	mv	a3,s3
     ae8:	00000097          	auipc	ra,0x0
     aec:	150080e7          	jalr	336(ra) # c38 <_ZN4core3fmt9Formatter12pad_integral12write_prefix17ha16bc6d3c7950701E>
     af0:	08051c63          	bnez	a0,b88 <_ZN4core3fmt9Formatter12pad_integral17h53dd5693591e204cE+0x310>
     af4:	01c42583          	lw	a1,28(s0)
     af8:	01842503          	lw	a0,24(s0)
     afc:	00c5a683          	lw	a3,12(a1)
     b00:	00090593          	mv	a1,s2
     b04:	00048613          	mv	a2,s1
     b08:	000680e7          	jalr	a3
     b0c:	06051e63          	bnez	a0,b88 <_ZN4core3fmt9Formatter12pad_integral17h53dd5693591e204cE+0x310>
     b10:	01842483          	lw	s1,24(s0)
     b14:	01c42403          	lw	s0,28(s0)
     b18:	00000913          	li	s2,0
     b1c:	112c0863          	beq	s8,s2,c2c <_ZN4core3fmt9Formatter12pad_integral17h53dd5693591e204cE+0x3b4>
     b20:	01042603          	lw	a2,16(s0)
     b24:	00190913          	addi	s2,s2,1
     b28:	00048513          	mv	a0,s1
     b2c:	000b8593          	mv	a1,s7
     b30:	000600e7          	jalr	a2
     b34:	fe0504e3          	beqz	a0,b1c <_ZN4core3fmt9Formatter12pad_integral17h53dd5693591e204cE+0x2a4>
     b38:	fff90513          	addi	a0,s2,-1
     b3c:	0f40006f          	j	c30 <_ZN4core3fmt9Formatter12pad_integral17h53dd5693591e204cE+0x3b8>
     b40:	00050b13          	mv	s6,a0
     b44:	00000513          	li	a0,0
     b48:	0100006f          	j	b58 <_ZN4core3fmt9Formatter12pad_integral17h53dd5693591e204cE+0x2e0>
     b4c:	00150593          	addi	a1,a0,1 # 110001 <__stack_size+0x10c001>
     b50:	00155513          	srli	a0,a0,0x1
     b54:	0015db13          	srli	s6,a1,0x1
     b58:	01842a03          	lw	s4,24(s0)
     b5c:	01c42a83          	lw	s5,28(s0)
     b60:	00442983          	lw	s3,4(s0)
     b64:	00150b93          	addi	s7,a0,1
     b68:	fffb8b93          	addi	s7,s7,-1
     b6c:	040b8c63          	beqz	s7,bc4 <_ZN4core3fmt9Formatter12pad_integral17h53dd5693591e204cE+0x34c>
     b70:	010aa603          	lw	a2,16(s5)
     b74:	000a0513          	mv	a0,s4
     b78:	00098593          	mv	a1,s3
     b7c:	000600e7          	jalr	a2
     b80:	fe0504e3          	beqz	a0,b68 <_ZN4core3fmt9Formatter12pad_integral17h53dd5693591e204cE+0x2f0>
     b84:	00100a93          	li	s5,1
     b88:	000a8513          	mv	a0,s5
     b8c:	02c12083          	lw	ra,44(sp)
     b90:	02812403          	lw	s0,40(sp)
     b94:	02412483          	lw	s1,36(sp)
     b98:	02012903          	lw	s2,32(sp)
     b9c:	01c12983          	lw	s3,28(sp)
     ba0:	01812a03          	lw	s4,24(sp)
     ba4:	01412a83          	lw	s5,20(sp)
     ba8:	01012b03          	lw	s6,16(sp)
     bac:	00c12b83          	lw	s7,12(sp)
     bb0:	00812c03          	lw	s8,8(sp)
     bb4:	00412c83          	lw	s9,4(sp)
     bb8:	00012d03          	lw	s10,0(sp)
     bbc:	03010113          	addi	sp,sp,48
     bc0:	00008067          	ret
     bc4:	00110537          	lui	a0,0x110
     bc8:	00100a93          	li	s5,1
     bcc:	faa98ee3          	beq	s3,a0,b88 <_ZN4core3fmt9Formatter12pad_integral17h53dd5693591e204cE+0x310>
     bd0:	01c42583          	lw	a1,28(s0)
     bd4:	01842503          	lw	a0,24(s0)
     bd8:	00c5a683          	lw	a3,12(a1)
     bdc:	00090593          	mv	a1,s2
     be0:	00048613          	mv	a2,s1
     be4:	000680e7          	jalr	a3
     be8:	fa0510e3          	bnez	a0,b88 <_ZN4core3fmt9Formatter12pad_integral17h53dd5693591e204cE+0x310>
     bec:	01842483          	lw	s1,24(s0)
     bf0:	01c42903          	lw	s2,28(s0)
     bf4:	00000a13          	li	s4,0
     bf8:	034b0263          	beq	s6,s4,c1c <_ZN4core3fmt9Formatter12pad_integral17h53dd5693591e204cE+0x3a4>
     bfc:	01092603          	lw	a2,16(s2)
     c00:	001a0a13          	addi	s4,s4,1 # 110001 <__stack_size+0x10c001>
     c04:	00048513          	mv	a0,s1
     c08:	00098593          	mv	a1,s3
     c0c:	000600e7          	jalr	a2
     c10:	fe0504e3          	beqz	a0,bf8 <_ZN4core3fmt9Formatter12pad_integral17h53dd5693591e204cE+0x380>
     c14:	fffa0513          	addi	a0,s4,-1
     c18:	f76568e3          	bltu	a0,s6,b88 <_ZN4core3fmt9Formatter12pad_integral17h53dd5693591e204cE+0x310>
     c1c:	00000a93          	li	s5,0
     c20:	01842223          	sw	s8,4(s0)
     c24:	03940023          	sb	s9,32(s0)
     c28:	f61ff06f          	j	b88 <_ZN4core3fmt9Formatter12pad_integral17h53dd5693591e204cE+0x310>
     c2c:	000c0513          	mv	a0,s8
     c30:	01853ab3          	sltu	s5,a0,s8
     c34:	f55ff06f          	j	b88 <_ZN4core3fmt9Formatter12pad_integral17h53dd5693591e204cE+0x310>

00000c38 <_ZN4core3fmt9Formatter12pad_integral12write_prefix17ha16bc6d3c7950701E>:
     c38:	ff010113          	addi	sp,sp,-16
     c3c:	00112623          	sw	ra,12(sp)
     c40:	00812423          	sw	s0,8(sp)
     c44:	00912223          	sw	s1,4(sp)
     c48:	01212023          	sw	s2,0(sp)
     c4c:	00110737          	lui	a4,0x110
     c50:	00068413          	mv	s0,a3
     c54:	00060493          	mv	s1,a2
     c58:	00050913          	mv	s2,a0
     c5c:	02e58063          	beq	a1,a4,c7c <_ZN4core3fmt9Formatter12pad_integral12write_prefix17ha16bc6d3c7950701E+0x44>
     c60:	01c92603          	lw	a2,28(s2)
     c64:	01892503          	lw	a0,24(s2)
     c68:	01062603          	lw	a2,16(a2)
     c6c:	000600e7          	jalr	a2
     c70:	00050593          	mv	a1,a0
     c74:	00100513          	li	a0,1
     c78:	02059c63          	bnez	a1,cb0 <_ZN4core3fmt9Formatter12pad_integral12write_prefix17ha16bc6d3c7950701E+0x78>
     c7c:	02048863          	beqz	s1,cac <_ZN4core3fmt9Formatter12pad_integral12write_prefix17ha16bc6d3c7950701E+0x74>
     c80:	01c92583          	lw	a1,28(s2)
     c84:	01892503          	lw	a0,24(s2)
     c88:	00c5a303          	lw	t1,12(a1)
     c8c:	00048593          	mv	a1,s1
     c90:	00040613          	mv	a2,s0
     c94:	00c12083          	lw	ra,12(sp)
     c98:	00812403          	lw	s0,8(sp)
     c9c:	00412483          	lw	s1,4(sp)
     ca0:	00012903          	lw	s2,0(sp)
     ca4:	01010113          	addi	sp,sp,16
     ca8:	00030067          	jr	t1
     cac:	00000513          	li	a0,0
     cb0:	00c12083          	lw	ra,12(sp)
     cb4:	00812403          	lw	s0,8(sp)
     cb8:	00412483          	lw	s1,4(sp)
     cbc:	00012903          	lw	s2,0(sp)
     cc0:	01010113          	addi	sp,sp,16
     cc4:	00008067          	ret

00000cc8 <_ZN4core3fmt3num3imp7fmt_u3217ha0c8e481e60c900bE.llvm.15097362283394770776>:
     cc8:	fa010113          	addi	sp,sp,-96
     ccc:	04112e23          	sw	ra,92(sp)
     cd0:	04812c23          	sw	s0,88(sp)
     cd4:	04912a23          	sw	s1,84(sp)
     cd8:	05212823          	sw	s2,80(sp)
     cdc:	05312623          	sw	s3,76(sp)
     ce0:	05412423          	sw	s4,72(sp)
     ce4:	05512223          	sw	s5,68(sp)
     ce8:	05612023          	sw	s6,64(sp)
     cec:	03712e23          	sw	s7,60(sp)
     cf0:	03812c23          	sw	s8,56(sp)
     cf4:	03912a23          	sw	s9,52(sp)
     cf8:	03a12823          	sw	s10,48(sp)
     cfc:	00060413          	mv	s0,a2
     d00:	00058493          	mv	s1,a1
     d04:	00050913          	mv	s2,a0
     d08:	00455513          	srli	a0,a0,0x4
     d0c:	27100593          	li	a1,625
     d10:	02700a13          	li	s4,39
     d14:	02b57663          	bgeu	a0,a1,d40 <_ZN4core3fmt3num3imp7fmt_u3217ha0c8e481e60c900bE.llvm.15097362283394770776+0x78>
     d18:	06300513          	li	a0,99
     d1c:	0d256e63          	bltu	a0,s2,df8 <_ZN4core3fmt3num3imp7fmt_u3217ha0c8e481e60c900bE.llvm.15097362283394770776+0x130>
     d20:	00a00513          	li	a0,10
     d24:	12a97c63          	bgeu	s2,a0,e5c <_ZN4core3fmt3num3imp7fmt_u3217ha0c8e481e60c900bE.llvm.15097362283394770776+0x194>
     d28:	fffa0513          	addi	a0,s4,-1
     d2c:	00910593          	addi	a1,sp,9
     d30:	00a585b3          	add	a1,a1,a0
     d34:	03090613          	addi	a2,s2,48
     d38:	00c58023          	sb	a2,0(a1)
     d3c:	14c0006f          	j	e88 <_ZN4core3fmt3num3imp7fmt_u3217ha0c8e481e60c900bE.llvm.15097362283394770776+0x1c0>
     d40:	00000a93          	li	s5,0
     d44:	00002537          	lui	a0,0x2
     d48:	71050993          	addi	s3,a0,1808 # 2710 <anon.a21e671ac1e8d09680791d1eb300dea1.15.llvm.6890862734925628598+0xdd8>
     d4c:	00001537          	lui	a0,0x1
     d50:	75c50b13          	addi	s6,a0,1884 # 175c <anon.70f5e8f8792695f3c09e486f406f2d91.5.llvm.577790792787444840>
     d54:	00910b93          	addi	s7,sp,9
     d58:	05f5e537          	lui	a0,0x5f5e
     d5c:	0ff50c13          	addi	s8,a0,255 # 5f5e0ff <__stack_size+0x5f5a0ff>
     d60:	00090a13          	mv	s4,s2
     d64:	00090513          	mv	a0,s2
     d68:	00098593          	mv	a1,s3
     d6c:	00000097          	auipc	ra,0x0
     d70:	5a4080e7          	jalr	1444(ra) # 1310 <__udivsi3>
     d74:	00050913          	mv	s2,a0
     d78:	00098593          	mv	a1,s3
     d7c:	00000097          	auipc	ra,0x0
     d80:	5bc080e7          	jalr	1468(ra) # 1338 <__mulsi3>
     d84:	40aa0cb3          	sub	s9,s4,a0
     d88:	010c9513          	slli	a0,s9,0x10
     d8c:	01055513          	srli	a0,a0,0x10
     d90:	06400593          	li	a1,100
     d94:	00000097          	auipc	ra,0x0
     d98:	57c080e7          	jalr	1404(ra) # 1310 <__udivsi3>
     d9c:	00151d13          	slli	s10,a0,0x1
     da0:	06400593          	li	a1,100
     da4:	00000097          	auipc	ra,0x0
     da8:	594080e7          	jalr	1428(ra) # 1338 <__mulsi3>
     dac:	40ac8533          	sub	a0,s9,a0
     db0:	01151513          	slli	a0,a0,0x11
     db4:	01055513          	srli	a0,a0,0x10
     db8:	016d05b3          	add	a1,s10,s6
     dbc:	015b8633          	add	a2,s7,s5
     dc0:	0005c683          	lbu	a3,0(a1)
     dc4:	00158583          	lb	a1,1(a1)
     dc8:	01650533          	add	a0,a0,s6
     dcc:	00150703          	lb	a4,1(a0)
     dd0:	00054503          	lbu	a0,0(a0)
     dd4:	02b60223          	sb	a1,36(a2)
     dd8:	02d601a3          	sb	a3,35(a2)
     ddc:	02e60323          	sb	a4,38(a2)
     de0:	02a602a3          	sb	a0,37(a2)
     de4:	ffca8a93          	addi	s5,s5,-4
     de8:	f74c6ce3          	bltu	s8,s4,d60 <_ZN4core3fmt3num3imp7fmt_u3217ha0c8e481e60c900bE.llvm.15097362283394770776+0x98>
     dec:	027a8a13          	addi	s4,s5,39
     df0:	06300513          	li	a0,99
     df4:	f32576e3          	bgeu	a0,s2,d20 <_ZN4core3fmt3num3imp7fmt_u3217ha0c8e481e60c900bE.llvm.15097362283394770776+0x58>
     df8:	01091513          	slli	a0,s2,0x10
     dfc:	01055513          	srli	a0,a0,0x10
     e00:	06400593          	li	a1,100
     e04:	00000097          	auipc	ra,0x0
     e08:	50c080e7          	jalr	1292(ra) # 1310 <__udivsi3>
     e0c:	00050993          	mv	s3,a0
     e10:	06400593          	li	a1,100
     e14:	00000097          	auipc	ra,0x0
     e18:	524080e7          	jalr	1316(ra) # 1338 <__mulsi3>
     e1c:	40a90533          	sub	a0,s2,a0
     e20:	01151513          	slli	a0,a0,0x11
     e24:	01055513          	srli	a0,a0,0x10
     e28:	ffea0a13          	addi	s4,s4,-2
     e2c:	000015b7          	lui	a1,0x1
     e30:	75c58593          	addi	a1,a1,1884 # 175c <anon.70f5e8f8792695f3c09e486f406f2d91.5.llvm.577790792787444840>
     e34:	00b50533          	add	a0,a0,a1
     e38:	00150583          	lb	a1,1(a0)
     e3c:	00054503          	lbu	a0,0(a0)
     e40:	00910613          	addi	a2,sp,9
     e44:	01460633          	add	a2,a2,s4
     e48:	00b600a3          	sb	a1,1(a2)
     e4c:	00a60023          	sb	a0,0(a2)
     e50:	00098913          	mv	s2,s3
     e54:	00a00513          	li	a0,10
     e58:	eca968e3          	bltu	s2,a0,d28 <_ZN4core3fmt3num3imp7fmt_u3217ha0c8e481e60c900bE.llvm.15097362283394770776+0x60>
     e5c:	00191593          	slli	a1,s2,0x1
     e60:	ffea0513          	addi	a0,s4,-2
     e64:	00001637          	lui	a2,0x1
     e68:	75c60613          	addi	a2,a2,1884 # 175c <anon.70f5e8f8792695f3c09e486f406f2d91.5.llvm.577790792787444840>
     e6c:	00c585b3          	add	a1,a1,a2
     e70:	00158603          	lb	a2,1(a1)
     e74:	0005c583          	lbu	a1,0(a1)
     e78:	00910693          	addi	a3,sp,9
     e7c:	00a686b3          	add	a3,a3,a0
     e80:	00c680a3          	sb	a2,1(a3)
     e84:	00b68023          	sb	a1,0(a3)
     e88:	00910593          	addi	a1,sp,9
     e8c:	00a58733          	add	a4,a1,a0
     e90:	02700593          	li	a1,39
     e94:	40a587b3          	sub	a5,a1,a0
     e98:	00002537          	lui	a0,0x2
     e9c:	82450613          	addi	a2,a0,-2012 # 1824 <.Lanon.be2ed760d3d5115759e7016d9dc85492.17>
     ea0:	00040513          	mv	a0,s0
     ea4:	00048593          	mv	a1,s1
     ea8:	00000693          	li	a3,0
     eac:	00000097          	auipc	ra,0x0
     eb0:	9cc080e7          	jalr	-1588(ra) # 878 <_ZN4core3fmt9Formatter12pad_integral17h53dd5693591e204cE>
     eb4:	05c12083          	lw	ra,92(sp)
     eb8:	05812403          	lw	s0,88(sp)
     ebc:	05412483          	lw	s1,84(sp)
     ec0:	05012903          	lw	s2,80(sp)
     ec4:	04c12983          	lw	s3,76(sp)
     ec8:	04812a03          	lw	s4,72(sp)
     ecc:	04412a83          	lw	s5,68(sp)
     ed0:	04012b03          	lw	s6,64(sp)
     ed4:	03c12b83          	lw	s7,60(sp)
     ed8:	03812c03          	lw	s8,56(sp)
     edc:	03412c83          	lw	s9,52(sp)
     ee0:	03012d03          	lw	s10,48(sp)
     ee4:	06010113          	addi	sp,sp,96
     ee8:	00008067          	ret

00000eec <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17hf32584490c374c5bE>:
     eec:	00052503          	lw	a0,0(a0)
     ef0:	00058613          	mv	a2,a1
     ef4:	00100593          	li	a1,1
     ef8:	00000317          	auipc	t1,0x0
     efc:	dd030067          	jr	-560(t1) # cc8 <_ZN4core3fmt3num3imp7fmt_u3217ha0c8e481e60c900bE.llvm.15097362283394770776>

00000f00 <_ZN4core3ptr28drop_in_place$LT$$RF$u64$GT$17h83656de8610feeb0E.llvm.15117363543361004793>:
     f00:	00008067          	ret

00000f04 <_ZN4core9panicking5panic17hcabcc163a9651165E>:
     f04:	fe010113          	addi	sp,sp,-32
     f08:	00a12c23          	sw	a0,24(sp)
     f0c:	00b12e23          	sw	a1,28(sp)
     f10:	01810513          	addi	a0,sp,24
     f14:	00a12023          	sw	a0,0(sp)
     f18:	00100513          	li	a0,1
     f1c:	00a12223          	sw	a0,4(sp)
     f20:	00012423          	sw	zero,8(sp)
     f24:	00002537          	lui	a0,0x2
     f28:	82450513          	addi	a0,a0,-2012 # 1824 <.Lanon.be2ed760d3d5115759e7016d9dc85492.17>
     f2c:	00a12823          	sw	a0,16(sp)
     f30:	00012a23          	sw	zero,20(sp)
     f34:	00010513          	mv	a0,sp
     f38:	00060593          	mv	a1,a2
     f3c:	00000097          	auipc	ra,0x0
     f40:	00c080e7          	jalr	12(ra) # f48 <_ZN4core9panicking9panic_fmt17h0a1f7e9b00d01db1E>
     f44:	c0001073          	unimp

00000f48 <_ZN4core9panicking9panic_fmt17h0a1f7e9b00d01db1E>:
     f48:	fe010113          	addi	sp,sp,-32
     f4c:	00002637          	lui	a2,0x2
     f50:	82460613          	addi	a2,a2,-2012 # 1824 <.Lanon.be2ed760d3d5115759e7016d9dc85492.17>
     f54:	00c12423          	sw	a2,8(sp)
     f58:	00002637          	lui	a2,0x2
     f5c:	82460613          	addi	a2,a2,-2012 # 1824 <.Lanon.be2ed760d3d5115759e7016d9dc85492.17>
     f60:	00c12623          	sw	a2,12(sp)
     f64:	00a12823          	sw	a0,16(sp)
     f68:	00b12a23          	sw	a1,20(sp)
     f6c:	00100513          	li	a0,1
     f70:	00a10c23          	sb	a0,24(sp)
     f74:	00810513          	addi	a0,sp,8
     f78:	fffff097          	auipc	ra,0xfffff
     f7c:	0a0080e7          	jalr	160(ra) # 18 <rust_begin_unwind>
     f80:	c0001073          	unimp

00000f84 <_ZN4core3ops8function6FnOnce9call_once17hc299f1191a3d291eE.llvm.5598078041313520213>:
     f84:	00000097          	auipc	ra,0x0
     f88:	31c080e7          	jalr	796(ra) # 12a0 <_ZN4core5slice5index27slice_end_index_len_fail_rt17hb26fa9c098a12081E>
     f8c:	c0001073          	unimp

00000f90 <_ZN4core10intrinsics17const_eval_select17h192bae309044d4e7E>:
     f90:	00000097          	auipc	ra,0x0
     f94:	ff4080e7          	jalr	-12(ra) # f84 <_ZN4core3ops8function6FnOnce9call_once17hc299f1191a3d291eE.llvm.5598078041313520213>
     f98:	c0001073          	unimp

00000f9c <_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h81f55edc6d7c53d7E>:
     f9c:	db5e1537          	lui	a0,0xdb5e1
     fa0:	f1850513          	addi	a0,a0,-232 # db5e0f18 <__heap_size+0x6b5e0f18>
     fa4:	bfd595b7          	lui	a1,0xbfd59
     fa8:	13558593          	addi	a1,a1,309 # bfd59135 <__heap_size+0x4fd59135>
     fac:	00008067          	ret

00000fb0 <_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$15copy_from_slice17len_mismatch_fail17h005c86f544d3a78cE>:
     fb0:	fd010113          	addi	sp,sp,-48
     fb4:	00a12023          	sw	a0,0(sp)
     fb8:	00b12223          	sw	a1,4(sp)
     fbc:	00410513          	addi	a0,sp,4
     fc0:	02a12023          	sw	a0,32(sp)
     fc4:	00001537          	lui	a0,0x1
     fc8:	eec50513          	addi	a0,a0,-276 # eec <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17hf32584490c374c5bE>
     fcc:	02a12223          	sw	a0,36(sp)
     fd0:	00010593          	mv	a1,sp
     fd4:	02b12423          	sw	a1,40(sp)
     fd8:	02a12623          	sw	a0,44(sp)
     fdc:	00002537          	lui	a0,0x2
     fe0:	87850513          	addi	a0,a0,-1928 # 1878 <.Lanon.f1f4ec4b6bfe469513882e68a08b4b70.23>
     fe4:	00a12423          	sw	a0,8(sp)
     fe8:	00300513          	li	a0,3
     fec:	00a12623          	sw	a0,12(sp)
     ff0:	00012823          	sw	zero,16(sp)
     ff4:	02010513          	addi	a0,sp,32
     ff8:	00a12c23          	sw	a0,24(sp)
     ffc:	00200513          	li	a0,2
    1000:	00a12e23          	sw	a0,28(sp)
    1004:	00810513          	addi	a0,sp,8
    1008:	00060593          	mv	a1,a2
    100c:	00000097          	auipc	ra,0x0
    1010:	f3c080e7          	jalr	-196(ra) # f48 <_ZN4core9panicking9panic_fmt17h0a1f7e9b00d01db1E>
    1014:	c0001073          	unimp

00001018 <_ZN4core3str5count14do_count_chars17h467d32cb0cb52437E>:
    1018:	fd010113          	addi	sp,sp,-48
    101c:	02112623          	sw	ra,44(sp)
    1020:	02812423          	sw	s0,40(sp)
    1024:	02912223          	sw	s1,36(sp)
    1028:	03212023          	sw	s2,32(sp)
    102c:	01312e23          	sw	s3,28(sp)
    1030:	01412c23          	sw	s4,24(sp)
    1034:	01512a23          	sw	s5,20(sp)
    1038:	01612823          	sw	s6,16(sp)
    103c:	01712623          	sw	s7,12(sp)
    1040:	01812423          	sw	s8,8(sp)
    1044:	00350613          	addi	a2,a0,3
    1048:	ffc67793          	andi	a5,a2,-4
    104c:	40a78733          	sub	a4,a5,a0
    1050:	00e5ee63          	bltu	a1,a4,106c <_ZN4core3str5count14do_count_chars17h467d32cb0cb52437E+0x54>
    1054:	40e58633          	sub	a2,a1,a4
    1058:	00463693          	sltiu	a3,a2,4
    105c:	00400813          	li	a6,4
    1060:	00e83833          	sltu	a6,a6,a4
    1064:	00d866b3          	or	a3,a6,a3
    1068:	02068663          	beqz	a3,1094 <_ZN4core3str5count14do_count_chars17h467d32cb0cb52437E+0x7c>
    106c:	00000413          	li	s0,0
    1070:	1e058863          	beqz	a1,1260 <_ZN4core3str5count14do_count_chars17h467d32cb0cb52437E+0x248>
    1074:	fbf00613          	li	a2,-65
    1078:	00050683          	lb	a3,0(a0)
    107c:	00150513          	addi	a0,a0,1
    1080:	00d626b3          	slt	a3,a2,a3
    1084:	fff58593          	addi	a1,a1,-1
    1088:	00d40433          	add	s0,s0,a3
    108c:	fe0596e3          	bnez	a1,1078 <_ZN4core3str5count14do_count_chars17h467d32cb0cb52437E+0x60>
    1090:	1d00006f          	j	1260 <_ZN4core3str5count14do_count_chars17h467d32cb0cb52437E+0x248>
    1094:	00367693          	andi	a3,a2,3
    1098:	00000593          	li	a1,0
    109c:	02070463          	beqz	a4,10c4 <_ZN4core3str5count14do_count_chars17h467d32cb0cb52437E+0xac>
    10a0:	40f507b3          	sub	a5,a0,a5
    10a4:	fbf00813          	li	a6,-65
    10a8:	00050893          	mv	a7,a0
    10ac:	00088283          	lb	t0,0(a7)
    10b0:	00188893          	addi	a7,a7,1
    10b4:	005822b3          	slt	t0,a6,t0
    10b8:	00178793          	addi	a5,a5,1
    10bc:	005585b3          	add	a1,a1,t0
    10c0:	fe0796e3          	bnez	a5,10ac <_ZN4core3str5count14do_count_chars17h467d32cb0cb52437E+0x94>
    10c4:	00e50933          	add	s2,a0,a4
    10c8:	00000513          	li	a0,0
    10cc:	02068463          	beqz	a3,10f4 <_ZN4core3str5count14do_count_chars17h467d32cb0cb52437E+0xdc>
    10d0:	ffc67713          	andi	a4,a2,-4
    10d4:	00e907b3          	add	a5,s2,a4
    10d8:	fbf00713          	li	a4,-65
    10dc:	00078803          	lb	a6,0(a5)
    10e0:	00178793          	addi	a5,a5,1
    10e4:	01072833          	slt	a6,a4,a6
    10e8:	fff68693          	addi	a3,a3,-1
    10ec:	01050533          	add	a0,a0,a6
    10f0:	fe0696e3          	bnez	a3,10dc <_ZN4core3str5count14do_count_chars17h467d32cb0cb52437E+0xc4>
    10f4:	00265b93          	srli	s7,a2,0x2
    10f8:	01010637          	lui	a2,0x1010
    10fc:	10160a13          	addi	s4,a2,257 # 1010101 <__stack_size+0x100c101>
    1100:	00ff0637          	lui	a2,0xff0
    1104:	0ff60993          	addi	s3,a2,255 # ff00ff <__stack_size+0xfec0ff>
    1108:	00010637          	lui	a2,0x10
    110c:	00160493          	addi	s1,a2,1 # 10001 <__stack_size+0xc001>
    1110:	00b50433          	add	s0,a0,a1
    1114:	03c0006f          	j	1150 <_ZN4core3str5count14do_count_chars17h467d32cb0cb52437E+0x138>
    1118:	00251593          	slli	a1,a0,0x2
    111c:	00b90933          	add	s2,s2,a1
    1120:	40aa8bb3          	sub	s7,s5,a0
    1124:	00357c13          	andi	s8,a0,3
    1128:	01367533          	and	a0,a2,s3
    112c:	00865593          	srli	a1,a2,0x8
    1130:	0135f5b3          	and	a1,a1,s3
    1134:	00a58533          	add	a0,a1,a0
    1138:	00048593          	mv	a1,s1
    113c:	00000097          	auipc	ra,0x0
    1140:	1fc080e7          	jalr	508(ra) # 1338 <__mulsi3>
    1144:	01055513          	srli	a0,a0,0x10
    1148:	00850433          	add	s0,a0,s0
    114c:	0a0c1863          	bnez	s8,11fc <_ZN4core3str5count14do_count_chars17h467d32cb0cb52437E+0x1e4>
    1150:	100b8863          	beqz	s7,1260 <_ZN4core3str5count14do_count_chars17h467d32cb0cb52437E+0x248>
    1154:	000b8a93          	mv	s5,s7
    1158:	0c000593          	li	a1,192
    115c:	000b8513          	mv	a0,s7
    1160:	00bbe463          	bltu	s7,a1,1168 <_ZN4core3str5count14do_count_chars17h467d32cb0cb52437E+0x150>
    1164:	0c000513          	li	a0,192
    1168:	0fc57593          	andi	a1,a0,252
    116c:	00259593          	slli	a1,a1,0x2
    1170:	00b90b33          	add	s6,s2,a1
    1174:	00000613          	li	a2,0
    1178:	fa0580e3          	beqz	a1,1118 <_ZN4core3str5count14do_count_chars17h467d32cb0cb52437E+0x100>
    117c:	00090593          	mv	a1,s2
    1180:	0005a683          	lw	a3,0(a1)
    1184:	fff6c713          	not	a4,a3
    1188:	00775713          	srli	a4,a4,0x7
    118c:	0066d693          	srli	a3,a3,0x6
    1190:	0045a783          	lw	a5,4(a1)
    1194:	00d766b3          	or	a3,a4,a3
    1198:	0146f6b3          	and	a3,a3,s4
    119c:	00c68633          	add	a2,a3,a2
    11a0:	fff7c693          	not	a3,a5
    11a4:	0076d693          	srli	a3,a3,0x7
    11a8:	0067d713          	srli	a4,a5,0x6
    11ac:	0085a783          	lw	a5,8(a1)
    11b0:	00e6e6b3          	or	a3,a3,a4
    11b4:	0146f6b3          	and	a3,a3,s4
    11b8:	00c68633          	add	a2,a3,a2
    11bc:	fff7c693          	not	a3,a5
    11c0:	0076d693          	srli	a3,a3,0x7
    11c4:	0067d713          	srli	a4,a5,0x6
    11c8:	00c5a783          	lw	a5,12(a1)
    11cc:	00e6e6b3          	or	a3,a3,a4
    11d0:	0146f6b3          	and	a3,a3,s4
    11d4:	00c68633          	add	a2,a3,a2
    11d8:	fff7c693          	not	a3,a5
    11dc:	0076d693          	srli	a3,a3,0x7
    11e0:	0067d713          	srli	a4,a5,0x6
    11e4:	00e6e6b3          	or	a3,a3,a4
    11e8:	0146f6b3          	and	a3,a3,s4
    11ec:	01058593          	addi	a1,a1,16
    11f0:	00c68633          	add	a2,a3,a2
    11f4:	f96596e3          	bne	a1,s6,1180 <_ZN4core3str5count14do_count_chars17h467d32cb0cb52437E+0x168>
    11f8:	f21ff06f          	j	1118 <_ZN4core3str5count14do_count_chars17h467d32cb0cb52437E+0x100>
    11fc:	0c000513          	li	a0,192
    1200:	00aae463          	bltu	s5,a0,1208 <_ZN4core3str5count14do_count_chars17h467d32cb0cb52437E+0x1f0>
    1204:	0c000a93          	li	s5,192
    1208:	00000513          	li	a0,0
    120c:	003af593          	andi	a1,s5,3
    1210:	00259593          	slli	a1,a1,0x2
    1214:	000b2603          	lw	a2,0(s6)
    1218:	004b0b13          	addi	s6,s6,4
    121c:	fff64693          	not	a3,a2
    1220:	0076d693          	srli	a3,a3,0x7
    1224:	00665613          	srli	a2,a2,0x6
    1228:	00c6e633          	or	a2,a3,a2
    122c:	01467633          	and	a2,a2,s4
    1230:	ffc58593          	addi	a1,a1,-4
    1234:	00a60533          	add	a0,a2,a0
    1238:	fc059ee3          	bnez	a1,1214 <_ZN4core3str5count14do_count_chars17h467d32cb0cb52437E+0x1fc>
    123c:	013575b3          	and	a1,a0,s3
    1240:	00855513          	srli	a0,a0,0x8
    1244:	01357533          	and	a0,a0,s3
    1248:	00b50533          	add	a0,a0,a1
    124c:	00048593          	mv	a1,s1
    1250:	00000097          	auipc	ra,0x0
    1254:	0e8080e7          	jalr	232(ra) # 1338 <__mulsi3>
    1258:	01055513          	srli	a0,a0,0x10
    125c:	00850433          	add	s0,a0,s0
    1260:	00040513          	mv	a0,s0
    1264:	02c12083          	lw	ra,44(sp)
    1268:	02812403          	lw	s0,40(sp)
    126c:	02412483          	lw	s1,36(sp)
    1270:	02012903          	lw	s2,32(sp)
    1274:	01c12983          	lw	s3,28(sp)
    1278:	01812a03          	lw	s4,24(sp)
    127c:	01412a83          	lw	s5,20(sp)
    1280:	01012b03          	lw	s6,16(sp)
    1284:	00c12b83          	lw	s7,12(sp)
    1288:	00812c03          	lw	s8,8(sp)
    128c:	03010113          	addi	sp,sp,48
    1290:	00008067          	ret

00001294 <_ZN4core5slice5index24slice_end_index_len_fail17hd8a83a9c6ff53337E>:
    1294:	00000097          	auipc	ra,0x0
    1298:	cfc080e7          	jalr	-772(ra) # f90 <_ZN4core10intrinsics17const_eval_select17h192bae309044d4e7E>
    129c:	c0001073          	unimp

000012a0 <_ZN4core5slice5index27slice_end_index_len_fail_rt17hb26fa9c098a12081E>:
    12a0:	fd010113          	addi	sp,sp,-48
    12a4:	00a12023          	sw	a0,0(sp)
    12a8:	00b12223          	sw	a1,4(sp)
    12ac:	00010513          	mv	a0,sp
    12b0:	02a12023          	sw	a0,32(sp)
    12b4:	00001537          	lui	a0,0x1
    12b8:	eec50513          	addi	a0,a0,-276 # eec <_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17hf32584490c374c5bE>
    12bc:	02a12223          	sw	a0,36(sp)
    12c0:	00410593          	addi	a1,sp,4
    12c4:	02b12423          	sw	a1,40(sp)
    12c8:	02a12623          	sw	a0,44(sp)
    12cc:	00002537          	lui	a0,0x2
    12d0:	92850513          	addi	a0,a0,-1752 # 1928 <anon.a21e671ac1e8d09680791d1eb300dea1.14.llvm.6890862734925628598>
    12d4:	00a12423          	sw	a0,8(sp)
    12d8:	00200513          	li	a0,2
    12dc:	00a12623          	sw	a0,12(sp)
    12e0:	00012823          	sw	zero,16(sp)
    12e4:	02010593          	addi	a1,sp,32
    12e8:	00b12c23          	sw	a1,24(sp)
    12ec:	00a12e23          	sw	a0,28(sp)
    12f0:	00002537          	lui	a0,0x2
    12f4:	93850593          	addi	a1,a0,-1736 # 1938 <anon.a21e671ac1e8d09680791d1eb300dea1.15.llvm.6890862734925628598>
    12f8:	00810513          	addi	a0,sp,8
    12fc:	00000097          	auipc	ra,0x0
    1300:	c4c080e7          	jalr	-948(ra) # f48 <_ZN4core9panicking9panic_fmt17h0a1f7e9b00d01db1E>
    1304:	c0001073          	unimp

00001308 <memcpy>:
    1308:	00000317          	auipc	t1,0x0
    130c:	07030067          	jr	112(t1) # 1378 <_ZN17compiler_builtins3mem6memcpy17h0f528ce3645dfde9E>

00001310 <__udivsi3>:
    1310:	00000317          	auipc	t1,0x0
    1314:	14c30067          	jr	332(t1) # 145c <_ZN17compiler_builtins3int19specialized_div_rem11u32_div_rem17hf8f96320097a68c6E>

00001318 <__umodsi3>:
    1318:	ff010113          	addi	sp,sp,-16
    131c:	00112623          	sw	ra,12(sp)
    1320:	00000097          	auipc	ra,0x0
    1324:	13c080e7          	jalr	316(ra) # 145c <_ZN17compiler_builtins3int19specialized_div_rem11u32_div_rem17hf8f96320097a68c6E>
    1328:	00058513          	mv	a0,a1
    132c:	00c12083          	lw	ra,12(sp)
    1330:	01010113          	addi	sp,sp,16
    1334:	00008067          	ret

00001338 <__mulsi3>:
    1338:	02050c63          	beqz	a0,1370 <__mulsi3+0x38>
    133c:	00050613          	mv	a2,a0
    1340:	00000513          	li	a0,0
    1344:	00100693          	li	a3,1
    1348:	0100006f          	j	1358 <__mulsi3+0x20>
    134c:	00175613          	srli	a2,a4,0x1
    1350:	00159593          	slli	a1,a1,0x1
    1354:	00e6fc63          	bgeu	a3,a4,136c <__mulsi3+0x34>
    1358:	00060713          	mv	a4,a2
    135c:	00167613          	andi	a2,a2,1
    1360:	fe0606e3          	beqz	a2,134c <__mulsi3+0x14>
    1364:	00b50533          	add	a0,a0,a1
    1368:	fe5ff06f          	j	134c <__mulsi3+0x14>
    136c:	00008067          	ret
    1370:	00000513          	li	a0,0
    1374:	00008067          	ret

00001378 <_ZN17compiler_builtins3mem6memcpy17h0f528ce3645dfde9E>:
    1378:	00f00693          	li	a3,15
    137c:	08c6f863          	bgeu	a3,a2,140c <_ZN17compiler_builtins3mem6memcpy17h0f528ce3645dfde9E+0x94>
    1380:	40a006b3          	neg	a3,a0
    1384:	0036f693          	andi	a3,a3,3
    1388:	00d50733          	add	a4,a0,a3
    138c:	02068063          	beqz	a3,13ac <_ZN17compiler_builtins3mem6memcpy17h0f528ce3645dfde9E+0x34>
    1390:	00050793          	mv	a5,a0
    1394:	00058813          	mv	a6,a1
    1398:	00080883          	lb	a7,0(a6)
    139c:	01178023          	sb	a7,0(a5)
    13a0:	00178793          	addi	a5,a5,1
    13a4:	00180813          	addi	a6,a6,1
    13a8:	fee7e8e3          	bltu	a5,a4,1398 <_ZN17compiler_builtins3mem6memcpy17h0f528ce3645dfde9E+0x20>
    13ac:	00d585b3          	add	a1,a1,a3
    13b0:	40d60633          	sub	a2,a2,a3
    13b4:	ffc67793          	andi	a5,a2,-4
    13b8:	0035f813          	andi	a6,a1,3
    13bc:	00f706b3          	add	a3,a4,a5
    13c0:	04080c63          	beqz	a6,1418 <_ZN17compiler_builtins3mem6memcpy17h0f528ce3645dfde9E+0xa0>
    13c4:	ffc5f313          	andi	t1,a1,-4
    13c8:	00032883          	lw	a7,0(t1)
    13cc:	06f05463          	blez	a5,1434 <_ZN17compiler_builtins3mem6memcpy17h0f528ce3645dfde9E+0xbc>
    13d0:	00359293          	slli	t0,a1,0x3
    13d4:	0182f813          	andi	a6,t0,24
    13d8:	405002b3          	neg	t0,t0
    13dc:	0182f293          	andi	t0,t0,24
    13e0:	00430313          	addi	t1,t1,4
    13e4:	00032383          	lw	t2,0(t1)
    13e8:	0108d8b3          	srl	a7,a7,a6
    13ec:	00539e33          	sll	t3,t2,t0
    13f0:	011e68b3          	or	a7,t3,a7
    13f4:	01172023          	sw	a7,0(a4) # 110000 <__stack_size+0x10c000>
    13f8:	00470713          	addi	a4,a4,4
    13fc:	00430313          	addi	t1,t1,4
    1400:	00038893          	mv	a7,t2
    1404:	fed760e3          	bltu	a4,a3,13e4 <_ZN17compiler_builtins3mem6memcpy17h0f528ce3645dfde9E+0x6c>
    1408:	02c0006f          	j	1434 <_ZN17compiler_builtins3mem6memcpy17h0f528ce3645dfde9E+0xbc>
    140c:	00050693          	mv	a3,a0
    1410:	02061863          	bnez	a2,1440 <_ZN17compiler_builtins3mem6memcpy17h0f528ce3645dfde9E+0xc8>
    1414:	0440006f          	j	1458 <_ZN17compiler_builtins3mem6memcpy17h0f528ce3645dfde9E+0xe0>
    1418:	00f05e63          	blez	a5,1434 <_ZN17compiler_builtins3mem6memcpy17h0f528ce3645dfde9E+0xbc>
    141c:	00058813          	mv	a6,a1
    1420:	00082883          	lw	a7,0(a6)
    1424:	01172023          	sw	a7,0(a4)
    1428:	00470713          	addi	a4,a4,4
    142c:	00480813          	addi	a6,a6,4
    1430:	fed768e3          	bltu	a4,a3,1420 <_ZN17compiler_builtins3mem6memcpy17h0f528ce3645dfde9E+0xa8>
    1434:	00f585b3          	add	a1,a1,a5
    1438:	00367613          	andi	a2,a2,3
    143c:	00060e63          	beqz	a2,1458 <_ZN17compiler_builtins3mem6memcpy17h0f528ce3645dfde9E+0xe0>
    1440:	00c68633          	add	a2,a3,a2
    1444:	00058703          	lb	a4,0(a1)
    1448:	00e68023          	sb	a4,0(a3)
    144c:	00168693          	addi	a3,a3,1
    1450:	00158593          	addi	a1,a1,1
    1454:	fec6e8e3          	bltu	a3,a2,1444 <_ZN17compiler_builtins3mem6memcpy17h0f528ce3645dfde9E+0xcc>
    1458:	00008067          	ret

0000145c <_ZN17compiler_builtins3int19specialized_div_rem11u32_div_rem17hf8f96320097a68c6E>:
    145c:	00050613          	mv	a2,a0
    1460:	00b57863          	bgeu	a0,a1,1470 <_ZN17compiler_builtins3int19specialized_div_rem11u32_div_rem17hf8f96320097a68c6E+0x14>
    1464:	00000513          	li	a0,0
    1468:	00060593          	mv	a1,a2
    146c:	00008067          	ret
    1470:	01065693          	srli	a3,a2,0x10
    1474:	00b6b533          	sltu	a0,a3,a1
    1478:	00154513          	xori	a0,a0,1
    147c:	00060713          	mv	a4,a2
    1480:	00b6e463          	bltu	a3,a1,1488 <_ZN17compiler_builtins3int19specialized_div_rem11u32_div_rem17hf8f96320097a68c6E+0x2c>
    1484:	00068713          	mv	a4,a3
    1488:	00875693          	srli	a3,a4,0x8
    148c:	00451513          	slli	a0,a0,0x4
    1490:	02b6e263          	bltu	a3,a1,14b4 <_ZN17compiler_builtins3int19specialized_div_rem11u32_div_rem17hf8f96320097a68c6E+0x58>
    1494:	00856513          	ori	a0,a0,8
    1498:	0046d713          	srli	a4,a3,0x4
    149c:	02b77263          	bgeu	a4,a1,14c0 <_ZN17compiler_builtins3int19specialized_div_rem11u32_div_rem17hf8f96320097a68c6E+0x64>
    14a0:	00068713          	mv	a4,a3
    14a4:	00275693          	srli	a3,a4,0x2
    14a8:	02b6f263          	bgeu	a3,a1,14cc <_ZN17compiler_builtins3int19specialized_div_rem11u32_div_rem17hf8f96320097a68c6E+0x70>
    14ac:	00070693          	mv	a3,a4
    14b0:	0200006f          	j	14d0 <_ZN17compiler_builtins3int19specialized_div_rem11u32_div_rem17hf8f96320097a68c6E+0x74>
    14b4:	00070693          	mv	a3,a4
    14b8:	0046d713          	srli	a4,a3,0x4
    14bc:	feb762e3          	bltu	a4,a1,14a0 <_ZN17compiler_builtins3int19specialized_div_rem11u32_div_rem17hf8f96320097a68c6E+0x44>
    14c0:	00456513          	ori	a0,a0,4
    14c4:	00275693          	srli	a3,a4,0x2
    14c8:	feb6e2e3          	bltu	a3,a1,14ac <_ZN17compiler_builtins3int19specialized_div_rem11u32_div_rem17hf8f96320097a68c6E+0x50>
    14cc:	00256513          	ori	a0,a0,2
    14d0:	0016d693          	srli	a3,a3,0x1
    14d4:	00b6b6b3          	sltu	a3,a3,a1
    14d8:	0016c693          	xori	a3,a3,1
    14dc:	00d566b3          	or	a3,a0,a3
    14e0:	00d59733          	sll	a4,a1,a3
    14e4:	40e60633          	sub	a2,a2,a4
    14e8:	00100513          	li	a0,1
    14ec:	00d51533          	sll	a0,a0,a3
    14f0:	06b66e63          	bltu	a2,a1,156c <_ZN17compiler_builtins3int19specialized_div_rem11u32_div_rem17hf8f96320097a68c6E+0x110>
    14f4:	00074663          	bltz	a4,1500 <_ZN17compiler_builtins3int19specialized_div_rem11u32_div_rem17hf8f96320097a68c6E+0xa4>
    14f8:	00050793          	mv	a5,a0
    14fc:	0300006f          	j	152c <_ZN17compiler_builtins3int19specialized_div_rem11u32_div_rem17hf8f96320097a68c6E+0xd0>
    1500:	00175713          	srli	a4,a4,0x1
    1504:	fff68693          	addi	a3,a3,-1
    1508:	00100793          	li	a5,1
    150c:	40e60833          	sub	a6,a2,a4
    1510:	00d797b3          	sll	a5,a5,a3
    1514:	00085663          	bgez	a6,1520 <_ZN17compiler_builtins3int19specialized_div_rem11u32_div_rem17hf8f96320097a68c6E+0xc4>
    1518:	00060813          	mv	a6,a2
    151c:	00c0006f          	j	1528 <_ZN17compiler_builtins3int19specialized_div_rem11u32_div_rem17hf8f96320097a68c6E+0xcc>
    1520:	00f56533          	or	a0,a0,a5
    1524:	00080613          	mv	a2,a6
    1528:	04b86263          	bltu	a6,a1,156c <_ZN17compiler_builtins3int19specialized_div_rem11u32_div_rem17hf8f96320097a68c6E+0x110>
    152c:	fff78593          	addi	a1,a5,-1
    1530:	02068863          	beqz	a3,1560 <_ZN17compiler_builtins3int19specialized_div_rem11u32_div_rem17hf8f96320097a68c6E+0x104>
    1534:	00100793          	li	a5,1
    1538:	40e78733          	sub	a4,a5,a4
    153c:	00068793          	mv	a5,a3
    1540:	00c0006f          	j	154c <_ZN17compiler_builtins3int19specialized_div_rem11u32_div_rem17hf8f96320097a68c6E+0xf0>
    1544:	fff78793          	addi	a5,a5,-1
    1548:	00078c63          	beqz	a5,1560 <_ZN17compiler_builtins3int19specialized_div_rem11u32_div_rem17hf8f96320097a68c6E+0x104>
    154c:	00161613          	slli	a2,a2,0x1
    1550:	00e60833          	add	a6,a2,a4
    1554:	fe0848e3          	bltz	a6,1544 <_ZN17compiler_builtins3int19specialized_div_rem11u32_div_rem17hf8f96320097a68c6E+0xe8>
    1558:	00080613          	mv	a2,a6
    155c:	fe9ff06f          	j	1544 <_ZN17compiler_builtins3int19specialized_div_rem11u32_div_rem17hf8f96320097a68c6E+0xe8>
    1560:	00b675b3          	and	a1,a2,a1
    1564:	00a5e533          	or	a0,a1,a0
    1568:	00d65633          	srl	a2,a2,a3
    156c:	00060593          	mv	a1,a2
    1570:	00008067          	ret
