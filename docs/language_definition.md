# Overview of the TROY Language 

The `TROY` ( **T**extual **R**epresentation **O**f crime stor**Y**)) is based on the [POLE model](https://www.npcc.police.uk/SysSiteAssets/media/downloads/publications/disclosure-logs/dei-coordination-committee/2023/274-2023-pole-data-standards-catalogue-v1.1-1-1.pdf) [¹]. 

The TROY language is designed so that ot can be quickly typed in whilst watching (or reading) a crime story. As such, keywords are very short and relationships are delimited with two or more spaces (no need to type in a special character).

It is easiest to explain this with an example 

For instance from the following description (from "Death's Shadow" S2E1)

    The village of Badger's Drift is again at the centre of attention when popular local property developer, Richard Bayly, is found decapitated in his own home. Richard had recently been diagnosed with a malignant brain tumour, and had been contemplating his likely premature death. The murder weapon, an Indian sword, is found near his body, and it turns out it was stolen in a burglary at the vicarage. Barnaby and Troy set out to find out who would go that far to kill a dying man. The police detectives realise that the key to the case is the death of a young local schoolboy years in the past. The truth finally emerges, but not until two more deaths occur, first when a handyman burns alive in his caravan, and second when another parishioner is struck by an arrow at a village fete. 

The following `TROY` description is found  (note `TROY` means **T**extual **R**epresentation **O**f crime stor**Y**)

```
In the TROY language:

pv Richard Bayly  found   l Own Home

pv Richard Bayly  aka  Richard

pv Richard   diagnosed with  o malignant tumor

ow Indian Sword   found near    pv Richard

ow Indian Sword   stolen     l Vicarage

e Death t years ago    of   p Local Schoolboy 

pv Handyman   burnt in  l caravan

pv Parishioner  struck by arrow    l village fete
```


Notice how keywords are very short and relationships are delimited with two or more spaces. This is to make typing the information very quick.

An another example (from "Dead Man's Eleven" S2E3)

    Tara Cavendish is found beaten to death near a disused quarry belonging to her husband Robert Cavendish. Tara was seen leaving the house early in the morning, but when she failed to return from a walk with their dog, Robert starts to worry, and reports her missing. Barnaby and Troy start to investigate. Nine years ago, only weeks before Robert Cavendish decided to shut down his own quarry, an explosion occurred. Matthew Draper, an employee who works for Robert Cavendish, was killed in the accident. And as further inquiries ensue, Barnaby and Troy are drawn to investigate the death eighteen months previously of Robert's former housekeeper, Emily Beavis. Emily’s sister, Doreen Beavis, and witnesses Colin and Christine Cooper, may hold the key to the case. 

In the TROY language: 

```
pv Tara Cavendish    beaten to death near   l Disused Quarry 

l Disused Quarry   belongs to   p Robert Cavendish

l Disused Quarry   aka   l Quarry

p Robert Cavendish    husband of   pv Tara Cavendish

p Robert Cavendish aka Robert

pv Tara Caendish aka Tara

l house   home of   pv Tara

l house   home of  p Robert 

pv Tara    seen leaving t early morning  l house

pv Tara    fails to return to   l house

p Robert   report missing   pv Tara

p Robert   shut down t 9 years ago   l  Quarry 

e Explosion    at t shortly before shutdown   l Quarry  

pv Matthew Draper   killed in   e Explosion 

pv Matthew Draper    worked for   p Robert

p Robert    former house keeper    pv Emily Beavis

p Emily Deavis  t 10 months before  e Death 

p Emily Beavis   sister   p Doreen Beavis

p Colin Cooper    witness to   e ?

p Christine  Cooper    witness to   e ?
```

### Keywords:

| Keyword | Description |
|---------|------------|
| `p`     | Person |
| `pv`    | Person Victim |
| `l`     | Location |
| `o`     | Object |
| `ow`    | Object Weapon |
| `e`     | Event |
| `aka`   | Also known as |
| `t`     | Time |


# BNF

See [troy.bnf](troy.bnf)


# Footnotes

[¹] : However, the model used by the `Barnaby` programm and with the the `TROY` language allows more flexibility in how relationship between entities are defined.  







