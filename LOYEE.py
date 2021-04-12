#!/usr/bin/python3

print('''

LOYEE a.a


Loyee é apenas um protótico, podendo ser modificado e aperfeiçoado para diversas situaçoes!

Loyee lhe dara todas as combinacoes possiveis dos dados informados, voce pode gerar uma Wordlist contendo todas as combinacoes possiveis de determinados caracteres ou com dados especificos de um alvo, usando as informacoes pessoas dele, tal como: NOME, DIA DO NASCIMENTO, MES DO NASCIMENTO, ANO DO NASCIMENTO, HOBBYS, APELIDO, NOME DOS FILHOS, NOME DE USUARIO, entre outras!


Obs: Use as informacoes pessoas separadas por espacos, exemplo: Fernanda 01 04 1992 Nanda . E os caracteres tambem separado por espaços, exemplo:  t e r e s c a r a c



Use com responsabilidade!
by William.

''')


a=str(input('D a d o s : ')).split()
while(len(a) < 2):
    print('\nMe der mais dados :)\n')
    a=str(input('D a d o s : ')).split()


b=c=d=e=f=g=h=i=j=0
msg='\n\nBugs? Me envie dicas, correçoes e aprimoramentos!  \n\n<wmhacker009@gmail.com>\n'
sim='SsYy'
combinacoes=0
espaco=0
memoria=''

def funcao():
    global c, d, e, f, g, h, i, b
    if(c==len(a)):
        b=0
        c=0
        d+=1
    if(d==len(a)):
        e+=1
        d=0
    if(e==len(a)):
        f+=1
        e=0
    if(f==len(a)):
        g+=1
        f=0
    if(g==len(a)):
        h+=1
        g=0
    if(h==len(a)):
        i+=1
        h=0

def funcao2():
    global c, d, e, f, g, h, i, b
    if(c==len(a)):
        b=0
        c=0
        d+=1
    if(d==len(a)):
        e+=1
        d=0
    if(e==len(a)):
        f+=1
        e=0
    if(f==len(a)):
        g+=1
        f=0
    if(g==len(a)):
        h+=1
        g=0

def funcao3():
    global c, d, e, f, g, h, i, b
    if(c==len(a)):
        b=0
        c=0
        d+=1
    if(d==len(a)):
        e+=1
        d=0
    if(e==len(a)):
        f+=1
        e=0


for vezes in range(1,len(a)+1):
    combinacoes+=len(a)**vezes
    espaco+=(len(a)**vezes)*vezes

msg2=0
msg3=''
z=1024
espaco2=espaco+combinacoes


if(espaco2 < z**1):
    msg2=espaco2
    msg3=' Bytes'

else:
    local=1
    medidas='KB MB GB TB PB EB ZB YB Bb Gb Sb Pb Ab'.split()
    while(espaco2 > z**local):
        msg2=espaco2/z**local
        msg3=medidas[local-1]
        local+=1

while(True):

    confirmacao=str(input(f'\nSerao gerados {combinacoes} combinacoes, com um espaco de {"%.2f" % msg2} {msg3}, deseja continuar? (S/N): '))
    confirmacao=confirmacao in sim
    print('\n'+50*'=')
    print('Aguarde...')
    print(50*'=')
    if(confirmacao  ==  False):
        break

    while(d-1!=len(a)):
        if(d==0):
            open('LOYEE.txt', 'a').write(f'{a[b]}\n')
        else:
            open('LOYEE.txt', 'a').write(f'{a[d-1]+a[b]}\n')
        b+=1
        c+=1
        if(c==len(a)):
            b=0
            c=0
            d+=1
    if(len(a) <= 2):
        break
    d=0

    while(e!=len(a)):
        open('LOYEE.txt', 'a').write(f'{a[e]+a[d]+a[b]}\n')
        b+=1
        c+=1
        if(c==len(a)):
            b=0
            c=0
            d+=1
        if(d==len(a)):
           e+=1
           d=0
    if(len(a) <= 3):
        break
    e=0

    while(f!=len(a)):
        open('LOYEE.txt', 'a').write(f'{a[f]+a[e]+a[d]+a[b]}\n')
        b+=1
        c+=1
        funcao3()
    if(len(a) <=4):
        break
    f=0

    while(g!=len(a)):
        open('LOYEE.txt', 'a').write(f'{a[g]+a[f]+a[e]+a[d]+a[b]}\n')
        b+=1
        c+=1
        funcao3()
        if(f==len(a)):
            g+=1
            f=0
    if(len(a) <=5):
        break
    g=0

    while(h!=len(a)):
        open('LOYEE.txt', 'a').write(f'{a[h]+a[g]+a[f]+a[e]+a[d]+a[b]}\n')
        b+=1
        c+=1
        funcao2()
    if(len(a) <= 6):
        break
    h=0

    while(i!=len(a)):
        open('LOYEE.txt', 'a').write(f'{a[i]+a[h]+a[g]+a[f]+a[e]+a[d]+a[b]}\n')
        b+=1
        c+=1
        funcao()
    if(len(a) <= 7):
        break
    i=0


    while(j!=len(a)):
        open('LOYEE.txt', 'a').write(f'{a[j]+a[i]+a[h]+a[g]+a[f]+a[e]+a[d]+a[b]}\n')
        b+=1
        c+=1
        funcao()
        if(i==len(a)):
            j+=1
            i=0
    if(len(a) <= 8):
        break

    break


if(confirmacao == True):
    print(f'\nForam gerados {combinacoes} combinaçoes possiveis, no arquivo LOYEE.txt, com um espaco de {"%.2f" % msg2}{msg3}{msg}')
else:
    print(f'\nByeeeeee! ate mais! :) {msg}')


