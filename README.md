# About Grepper ([www.codegrepper.com](https://www.codegrepper.com))
Grepper is a platform to help developers solve technical problems. Grepper is powered by an amazing community of 200k(and growing fast) developers. Developers within the community contribute answers to technical problems primarily through the Grepper browser extension which allows a user to quickly add a “code snippet” as an answer to a problem they ran into and recently solved. At present the community has put in 318k answers, which together have been viewed over 62 million times.


# How Grepper Contributor Coin (GRPC) works.
Coins will be minted on a daily basis, to users who have contributed helpful answers/content to Grepper. Initially 1 GRPC will be issued per answer per day for answers that rank in the top 30% quality score.  Additionally, in order to prevent spam answers, for  “bad answers” (answers that have a quality score in the bottom 20% of answers) 1 GRPC will be deducted from that day's earnings. 

Ex: Sally has inputted 20 Grepper answers, 10 of which have a quality score that fall within the top 30% of answers and 2 of which fall within the bottom 20% of answers. Sally will earn 8 newly minted GRPC per day.  If Sally sets the 2 bad answers to private or deletes them, then those answers will not be deducted from her earnings, so she will earn 10 GRPC a day.  Users can’t earn negative coins daily, so if a user only has 2 answers and they are both bad, they will simply earn 0 GRPC a day. 

**Good Answers (Top 30%)**: At present in order to be in the top 30% an answer must have a quality score of .93 or about ~1. This number should increase slightly as the quality score of all answers increase. 

**Bad Answers (Bottom 20%)**: At present this number is a quality score of less than ~0. This number should increase slightly as the quality score of all answers increase. 

**Quality Score**: The thrust of an answers “quality score” is to get a number on how “helpful” an answer is to other developers, this is a little tricky and may be adjusted in order to better capture “helpfulness”. At present an answer's “quality score” is calculated based on a weighted number of upvotes, downvotes and copies it gets relative to views . Quality Score = (upvotes*upvoteWeight) + (copies*copyWeight) - (downvotes*downvoteWeight ) / (views) 



# The Donation System 
Anyone who is helped by a Grepper answer or wants to support the community can donate to all “Grepper Contributors” by donating to a “smart contract” found [here](https://github.com/CodeGrepper/grepper_contributor_coin_contract/tree/main/program-rust/src/lib.rs). This contract holds SOL and any holder of GRPC can swap their GRPC for SOL based on the amount of SOL held in the contract and the amount of GRPC outstanding.  Ex: Let’s say sally has 5,000 GRPC and wants to swap them for SOL. If at the time there are 1,000,000GRPC outstanding and the contract holds 10SOL.  Sally can Swap her 5k GRPC for .05 SOL (.05 = 10/1000000 * 5000) 

*Note: SOL can be swapped for almost any currency on a number of exchanges such as coinbase.*


