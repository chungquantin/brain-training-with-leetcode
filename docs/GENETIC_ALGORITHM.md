## Genetic Algorithm

### Classical GA

https://link.springer.com/content/pdf/10.1007/s11042-020-10139-6.pdf

**Genetic algorithm (GA)** is an optimization algorithm that is inspired from the natural selection. It is a population based search algorithm, which utilizes the concept of survival of fittest. The new populations are produced by iterative use of genetic operators on individuals present in the population. The chromosome representation, selection, crossover, mutation, and fitness function computation are the key elements of GA. The procedure of GA is as follows. A population (Y) of n chromosomes are initialized randomly. The fitness of each chromosome in Y is computed. Two chromosomes say C1 and C2 are selected from the population Y according to the fitness value.

The single-point crossover operator with crossover probability (Cp) is applied on C1 and C2 to produce an offspring say O. Thereafter, uniform mutation operator is applied on produced offspring (O) with mutation probability (Mp) to generate O′. The new offspring O′ is placed in new population. The selection, crossover, and mutation operations will be repeated on current population until the new population is complete.
The mathematical analysis of GA is as follows [126]:

GA dynamically change the search process through the probabilities of crossover and
mutation and reached to optimal solution. GA can modify the encoded genes. GA can evaluate
multiple individuals and produce multiple optimal solutions. Hence, GA has better global
search capability. The offspring produced from crossover of parent chromosomes is probable
to abolish the admirable genetic schemas parent chromosomes and crossover formula is
defined as [126]:
